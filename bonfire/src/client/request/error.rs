use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

use crate::Result;

pub(crate) trait RequestErrorSource: StdError + Send + Sync + 'static {}

pub(crate) trait RequestError: StdError + Send + Sync + 'static {
    type Source: RequestErrorSource;

    fn try_convert(error: &Self::Source) -> Result<Option<Self>>
    where
        Self: Sized;
}

// Requests that don't have a request-specific error should use this struct as the Error type
pub(crate) enum InfallibleRequest<S: RequestErrorSource> {
    _Marker(PhantomData<S>),
}

impl<S: RequestErrorSource> fmt::Debug for InfallibleRequest<S> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl<S: RequestErrorSource> fmt::Display for InfallibleRequest<S> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl<S: RequestErrorSource> StdError for InfallibleRequest<S> {}

impl<S: RequestErrorSource> RequestError for InfallibleRequest<S> {
    type Source = S;

    fn try_convert(_error: &Self::Source) -> Result<Option<Self>> {
        Ok(None)
    }
}
