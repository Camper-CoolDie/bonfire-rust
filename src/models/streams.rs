use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures::future::OptionFuture;
use futures::{Stream, StreamExt as _, stream};

use crate::Result;

type RequestFn<'a, O, T> =
    Box<dyn Fn(O) -> Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send + 'a>> + Send + Sync + 'a>;
type NextOffsetFn<'a, O, T> = Box<dyn Fn(&Vec<T>, O) -> Option<O> + Send + Sync + 'a>;

pub(super) fn paginated_stream<'a, O, T>(
    request_fn: RequestFn<'a, O, T>,
    start_offset: O,
    next_offset_fn: NextOffsetFn<'a, O, T>,
) -> impl Stream<Item = Result<T>>
where
    T: Send + 'a,
    O: Clone + Send + 'a,
{
    let request_fn = Arc::new(request_fn);
    let next_offset_fn = Arc::new(next_offset_fn);

    stream::unfold(Some(start_offset), move |offset_option| {
        let request_fn = request_fn.clone();
        let next_offset_fn = next_offset_fn.clone();

        async move {
            let result_option = OptionFuture::from(
                offset_option.map(|offset| async { (request_fn(offset.clone()).await, offset) }),
            );
            match result_option.await {
                Some((Ok(items), offset)) => {
                    let next_offset = next_offset_fn(&items, offset);

                    Some((
                        stream::iter(items.into_iter().map(Ok)).left_stream(),
                        next_offset,
                    ))
                }
                Some((Err(error), _)) => {
                    Some((stream::once(async { Err(error) }).right_stream(), None))
                }
                None => None,
            }
        }
    })
    .flatten()
}
