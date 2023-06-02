use async_stream::try_stream;
use example::streamer_client::*;
use example::*;
use futures::channel::mpsc::channel as futures_channel;
use futures::{SinkExt, Stream, StreamExt};
use std::pin::Pin;
use tokio::sync::mpsc::channel as tokio_channel;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::channel::Endpoint;
use tonic::{Request, Response, Status};
use tracing::{info, instrument};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{Layer, Registry};

#[instrument]
pub async fn run_client_requests() {
    let mut client = StreamerClient::connect(Endpoint::from_static("http://127.0.0.1:18080"))
        .await
        .unwrap();

    info!("Example 1");

    let mut stream = client
        .example1(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("Example 2");

    let mut stream = client
        .example2(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("Example 3");

    let mut stream = client
        .example3(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }
}

pub fn setup_logging() {
    let filter = EnvFilter::new("tonic_repo=info,client=info,server=info");

    let fmt = tracing_subscriber::fmt::Layer::default();

    let subscriber = filter.and_then(fmt).with_subscriber(Registry::default());

    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub mod example {
    tonic::include_proto!("protos.example");
}

#[derive(Clone)]
pub struct StreamerImpl;

#[tonic::async_trait]
impl streamer_server::Streamer for StreamerImpl {
    type example1Stream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type example2Stream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type example3Stream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;

    async fn example1(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::example1Stream>, Status> {
        info!("Example 1");
        let request = request.into_inner();

        let (mut tx, rx) = futures_channel(0);

        let _sender = tokio::spawn(async move {
            for _ in 0..request.packets {
                let mut new_message = Data {
                    values: Vec::with_capacity(request.packet_size as usize),
                };
                for _ in 0..request.packet_size {
                    new_message.values.push(fastrand::i32(..));
                }
                info!("Sending message");
                if tx.send(Ok(new_message)).await.is_err() {
                    break;
                }
            }
        });
        Ok(Response::new(Box::pin(rx) as Self::example1Stream))
    }

    async fn example2(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::example2Stream>, Status> {
        info!("Example 2");
        let request = request.into_inner();

        let output = try_stream! {
            for _ in 0..request.packets {
                let mut new_message = Data {
                    values: Vec::with_capacity(request.packet_size as usize)
                };
                for _ in 0..request.packet_size {
                    new_message.values.push(fastrand::i32(..));
                }
                info!("Sending message");
                yield new_message;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::example2Stream))
    }

    async fn example3(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::example3Stream>, Status> {
        info!("Example 3");
        let request = request.into_inner();

        let (tx, rx) = tokio_channel(10);

        let _sender = tokio::spawn(async move {
            for _ in 0..request.packets {
                let mut new_message = Data {
                    values: Vec::with_capacity(request.packet_size as usize),
                };
                for _ in 0..request.packet_size {
                    new_message.values.push(fastrand::i32(..));
                }
                info!("Sending message");
                if tx.send(Ok(new_message)).await.is_err() {
                    break;
                }
            }
        });
        let rx = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(rx) as Self::example3Stream))
    }
}
