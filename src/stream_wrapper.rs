use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::wrappers::ReceiverStream;
use tracing::info;

pub struct ReceiverStreamWrapper<T>(pub ReceiverStream<T>);

impl<T> Stream for ReceiverStreamWrapper<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        info!("Poll next called");
        Pin::new(&mut self.0).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        info!("Size hint called");
        self.0.size_hint()
    }
}
