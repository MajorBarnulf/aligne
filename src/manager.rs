use std::{collections::HashMap, fmt::Debug};

use tokio::sync::mpsc;

use crate::{message, ResponseManagerHandle};

/// The state of a request that has been stored.
#[derive(Debug)]
enum ResponseState<R> {
	Received(message::Receive<R>),
	Requested(message::Request<R>),
}

/// Encapsulates an asynchronous task that receives responses and requests asynchronously and matches them by id ([`u64`]).
/// once both the response and the request are received, the request complete its future and receive the response.
pub struct ResponseManager<R>
where
	R: Send + Debug,
{
	response: HashMap<u64, ResponseState<R>>,
}

impl<R> ResponseManager<R>
where
	R: Send + Debug + 'static,
{
	/// spawn a new [`self::ResponseManager`] and returns its task handle ([`crate::ResponseManagerHandle`]).
	pub fn spawn() -> ResponseManagerHandle<R> {
		let (sender, receiver) = mpsc::unbounded_channel();
		let task = tokio::spawn(async {
			Self::new().listen(receiver).await;
		});

		ResponseManagerHandle::new(task, sender)
	}

	fn new() -> Self {
		let response = HashMap::new();
		Self { response }
	}

	async fn listen(
		&mut self,
		mut receiver: mpsc::UnboundedReceiver<message::Message<R>>,
	) -> Option<()> {
		loop {
			let message = receiver.recv().await?;
			self.handle_message(message);
		}
	}

	fn handle_message(&mut self, message: message::Message<R>) {
		match message {
			message::Message::Request(request) => self.on_request(request),
			message::Message::Receive(store) => self.on_receive(store),
		}
	}

	fn on_request(&mut self, request: message::Request<R>) {
		let query = self.response.remove(&request.request_id);
		match query {
			None => self.store_request(request),
			Some(ResponseState::Received(receive)) => Self::fulfill_request(request, receive),
			Some(ResponseState::Requested(_)) => {
				panic!("requested two requests for the same ID")
			}
		}
	}

	fn on_receive(&mut self, receive: message::Receive<R>) {
		let query = self.response.remove(&receive.request_id);
		match query {
			None => self.store_received(receive),
			Some(ResponseState::Requested(request)) => Self::fulfill_request(request, receive),
			Some(ResponseState::Received(_)) => {
				panic!("received two response for the same ID")
			}
		}
	}

	fn store_request(&mut self, request: message::Request<R>) {
		let key = request.request_id;
		self.response.insert(key, ResponseState::Requested(request));
	}

	fn store_received(&mut self, received: message::Receive<R>) {
		let key = received.request_id;
		self.response.insert(key, ResponseState::Received(received));
	}

	fn fulfill_request(request: message::Request<R>, receive: message::Receive<R>) {
		let message::Request {
			request_id: _,
			resolve,
		} = request;
		let message::Receive {
			request_id: _,
			signal,
		} = receive;
		resolve.send(signal).unwrap();
	}
}
