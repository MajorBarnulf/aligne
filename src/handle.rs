use std::fmt::Debug;

use tokio::{sync::mpsc, task::JoinHandle};

use crate::{message, ResponseManagerRemote};

/// Handle to the task of a [`crate::ResponseManager`], can spawn [`self::ResponseManagerRemote`] to interact with the task.
pub struct ResponseManagerHandle<R>
where
	R: Send + Debug,
{
	task: JoinHandle<()>,
	sender: mpsc::UnboundedSender<message::Message<R>>,
}

impl<R> ResponseManagerHandle<R>
where
	R: Send + Debug,
{
	/// Constructor, ment to be acquired by spawning a [`crate::ResponseManager`].
	pub fn new(task: JoinHandle<()>, sender: mpsc::UnboundedSender<message::Message<R>>) -> Self {
		Self { task, sender }
	}

	/// Awaits the loop of the task.
	pub async fn join(self) {
		let Self { task, sender: _ } = self;
		task.await.unwrap();
	}

	/// Copies a shareable [`self::ResponseManagerRemote`] that can interact with the task.
	pub fn remote(&self) -> ResponseManagerRemote<R> {
		let sender = self.sender.clone();
		ResponseManagerRemote::new(sender)
	}
}
