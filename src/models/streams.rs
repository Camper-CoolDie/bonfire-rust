use std::future::Future;

use futures::future::OptionFuture;
use futures::{Stream, StreamExt as _, stream};

use crate::Result;

pub(super) fn paginated_stream<'a, T, O, F, N, Fut>(
    request_fn: F,
    start_offset: O,
    next_offset_fn: N,
) -> impl Stream<Item = Result<T>>
where
    T: Send + 'a,
    O: Clone + Send + 'a,
    F: Fn(O) -> Fut + Clone + Send + 'a,
    for<'b> N: Fn(&'b Vec<T>, O) -> Option<O> + Clone + Send + 'a,
    Fut: Future<Output = Result<Vec<T>>> + Send + 'a,
{
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
