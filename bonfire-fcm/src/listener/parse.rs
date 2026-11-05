use std::error::Error as StdError;
use std::result::Result as StdResult;

use crate::Error;
use crate::models::{Message, Subscription};

pub type Result<P> = StdResult<<P as Parse>::Target, <P as Parse>::Error>;

#[doc(hidden)]
pub trait Parse: Send + Sync + 'static {
    type Target: Send;
    type Error: StdError + Send;

    fn parse(&mut self, message: Message) -> StdResult<Option<Self::Target>, Self::Error>;
    fn stop(self, subscription: Subscription, error: Option<Error>);
}
