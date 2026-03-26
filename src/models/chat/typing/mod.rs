mod command;
mod handler;

use command::Command;
pub use handler::Handler;

use crate::Client;
use crate::models::Chat;
use crate::models::chat::Messageable as _;

impl Chat {
    /// Starts sending periodic typing notifications to this chat.
    ///
    /// This method spawns a background task that sends a typing notification to the specified chat.
    /// The returned [`TypingHandler`][Handler] can be used to pause and resume these notifications.
    /// The task will also stop automatically when the handler is dropped.
    #[must_use]
    pub fn start_typing(&self, client: &Client) -> Handler {
        Handler::spawn(client, self.kind.tag())
    }
}
