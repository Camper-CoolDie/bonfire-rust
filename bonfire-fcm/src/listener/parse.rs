use std::error::Error as StdError;
use std::result::Result as StdResult;

use crate::Error;
use crate::models::{Message, Subscription};

#[doc(hidden)]
pub trait Parse: Send + Sync + 'static {
    type Target: Send;
    type Error: From<Error> + StdError + Send;

    fn parse(
        &self,
        message: Message,
    ) -> impl Future<Output = StdResult<Option<Self::Target>, Self::Error>> + Send;

    fn stop(
        self,
        subscription: Subscription,
        error: Option<Self::Error>,
    ) -> impl Future<Output = ()> + Send;
}
