use std::future::Future;

use futures::future::OptionFuture;
use futures::{Stream, StreamExt as _, stream};

use crate::Result;

pub(super) fn paginated_stream<'a, O, T, R, N, Fut>(
    request_fn: R,
    offset: O,
    next_offset_fn: N,
) -> impl Stream<Item = Result<T>>
where
    O: Clone + Send + Sync + 'a,
    T: Send + Sync + 'a,
    R: Fn(O) -> Fut + Clone + Send + Sync + 'a,
    N: Fn(&Vec<T>, O) -> Option<O> + Clone + Send + Sync + 'a,
    Fut: Future<Output = Result<Vec<T>>> + Send + Sync + 'a,
{
    stream::unfold(Some(offset), move |offset_option| {
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

pub(super) fn auto_paginated_stream<'a, T, R, Fut>(
    request_fn: R,
    offset: usize,
    page_size: usize,
) -> impl Stream<Item = Result<T>>
where
    T: Send + Sync + 'a,
    R: Fn(usize) -> Fut + Clone + Send + Sync + 'a,
    Fut: Future<Output = Result<Vec<T>>> + Send + Sync + 'a,
{
    paginated_stream(request_fn, offset, move |items, offset| {
        let length = items.len();
        (length >= page_size).then_some(offset + length)
    })
}
