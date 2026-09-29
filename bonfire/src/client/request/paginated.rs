use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use futures::{FutureExt as _, Stream};

use crate::client::Request;
use crate::{Client, Result};

pub(crate) struct PaginatedRequest<'a, T, R: Request<Vec<T>> + 'a> {
    client: &'a Client,
    request: Option<R>,
    items: VecDeque<T>,
    future: Option<BoxFuture<'a, (Result<Vec<T>>, R)>>,
    offset: fn(&mut R, &[T]),
    page_size: usize,
}
impl<'a, T, R: Request<Vec<T>> + 'a> PaginatedRequest<'a, T, R> {
    pub(crate) fn new(
        client: &'a Client,
        request: R,
        offset: fn(&mut R, &[T]),
        page_size: NonZeroUsize,
    ) -> Self {
        let page_size = page_size.get();

        Self {
            client,
            request: Some(request),
            items: VecDeque::with_capacity(page_size),
            future: None,
            offset,
            page_size,
        }
    }
}

impl<T, R> Stream for PaginatedRequest<'_, T, R>
where
    T: Unpin,
    R: Request<Vec<T>> + Unpin,
{
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Result<T>>> {
        let this = self.get_mut();

        loop {
            if let Some(item) = this.items.pop_front() {
                // Drain received items
                return Poll::Ready(Some(Ok(item)));
            }

            if let Some(future) = this.future.as_mut() {
                match future.poll_unpin(ctx) {
                    Poll::Ready((Ok(items), mut request)) => {
                        this.future = None;

                        // The items length may be zero (in case previous page was the last one)
                        if items.len() >= this.page_size {
                            (this.offset)(&mut request, &items);
                            this.request = Some(request);
                        }
                        this.items.extend(items);
                        // The first item is immediately returned
                    }
                    Poll::Ready((Err(error), _)) => {
                        this.future = None;
                        this.request = None;
                        return Poll::Ready(Some(Err(error)));
                    }
                    Poll::Pending => return Poll::Pending,
                }
            } else {
                let Some(request) = this.request.take() else {
                    // Stream is exhausted
                    return Poll::Ready(None);
                };

                // Request next page
                let client = this.client;
                this.future = Some(
                    async move {
                        let result = request.send(client).await;
                        (result, request)
                    }
                    .boxed(),
                );
                // The future is immediately polled
            }
        }
    }
}
