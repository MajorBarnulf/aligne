//! # aligne
//!
//! A crate to help synchronizing asynchronous request / responses.
//!
//! ## Example:
//! ```
//! #[tokio::main]
//! pub async fn main() {
//!     let handle = aligne::ResponseManager::spawn();
//!     let remote_a = handle.remote();
//!     let remote_b = remote_a.clone();
//!
//!     let _h1 = tokio::spawn(async move {
//!         let msg = remote_b.request(2).await;
//!         assert_eq!(msg, "message 2");
//!         let msg = remote_b.request(1).await;
//!         assert_eq!(msg, "message 1");
//!     });
//!
//!     let _h2 = tokio::spawn(async move {
//!         remote_a.receive(1, "message 1").await;
//!         remote_a.receive(2, "message 2").await;
//!     });
//!
//!     _h1.await.unwrap();
//!     _h2.await.unwrap();
//! }
//! ```

pub use handle::ResponseManagerHandle;
pub use manager::ResponseManager;
pub use remote::ResponseManagerRemote;

mod handle;
mod manager;
mod message;
mod remote;
