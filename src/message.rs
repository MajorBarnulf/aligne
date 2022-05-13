use tokio::sync::oneshot;

/// Either kind of messages.
#[derive(Debug)]
pub enum Message<R> {
	Request(Request<R>),
	Receive(Receive<R>),
}

/// A received request.
#[derive(Debug)]
pub struct Request<R> {
	pub request_id: u64,
	pub resolve: oneshot::Sender<R>,
}

/// A received response.
#[derive(Debug)]
pub struct Receive<R> {
	pub request_id: u64,
	pub signal: R,
}
