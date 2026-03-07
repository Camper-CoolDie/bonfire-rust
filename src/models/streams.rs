use std::future::Future;

use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt as _, stream};

use crate::Result;

pub(super) fn paginated_stream<'a, T, F, Fut>(
    request_fn: F,
    start_offset: u64,
    page_size: usize,
) -> impl Stream<Item = Result<T>>
where
    T: Send + 'a,
    F: Fn(u64) -> Fut + Clone + Send + 'a,
    Fut: Future<Output = Result<Vec<T>>> + Send + 'a,
{
    stream::unfold(Some(start_offset), move |offset_option| {
        let request_fn = request_fn.clone();

        async move {
            if let Some(offset) = offset_option {
                match request_fn(offset).await {
                    Ok(items) => {
                        let page_length = items.len();
                        let next_offset =
                            (page_length >= page_size).then_some(offset + page_length as u64);

                        Some((
                            stream::iter(items.into_iter().map(Ok)).left_stream(),
                            next_offset,
                        ))
                    }
                    Err(error) => Some((stream::once(async { Err(error) }).right_stream(), None)),
                }
            } else {
                None
            }
        }
    })
    .flatten()
}

pub(super) fn paginated_by_date_stream<'a, T, F, N, Fut>(
    request_fn: F,
    start_offset_date: Option<DateTime<Utc>>,
    page_size: usize,
    next_offset_date_fn: N,
) -> impl Stream<Item = Result<T>>
where
    T: Send + 'a,
    F: Fn(Option<DateTime<Utc>>) -> Fut + Clone + Send + 'a,
    for<'b> N: Fn(&'b T) -> DateTime<Utc> + Clone + Send + 'a,
    Fut: Future<Output = Result<Vec<T>>> + Send + 'a,
{
    stream::unfold(Some(start_offset_date), move |offset_date_option| {
        let request_fn = request_fn.clone();
        let next_offset_date_fn = next_offset_date_fn.clone();

        async move {
            if let Some(offset_date) = offset_date_option {
                match request_fn(offset_date).await {
                    Ok(items) => {
                        let page_length = items.len();
                        let next_offset_date = (page_length >= page_size)
                            .then(|| Some(next_offset_date_fn(items.last().unwrap())));

                        Some((
                            stream::iter(items.into_iter().map(Ok)).left_stream(),
                            next_offset_date,
                        ))
                    }
                    Err(error) => Some((stream::once(async { Err(error) }).right_stream(), None)),
                }
            } else {
                None
            }
        }
    })
    .flatten()
}
