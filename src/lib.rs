use async_stream::try_stream;
use example::streamer_client::*;
use example::*;
use futures::channel::mpsc::channel as futures_channel;
use futures::{SinkExt, Stream, StreamExt};
use std::pin::Pin;
use tokio::sync::mpsc::channel as tokio_channel;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
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

    info!("Futures Channel");

    let mut stream = client
        .futures_channel(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("async_stream!");

    let mut stream = client
        .async_stream(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("tokio mpsc");

    let mut stream = client
        .tokio_mpsc(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("async-channel::bounded");

    let mut stream = client
        .async_channel(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("tokio unbounded mpsc");

    let mut stream = client
        .unbounded_tokio_mpsc(Input {
            packets: 20,
            packet_size: 1000,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(_data) = stream.next().await {
        info!("Receiving message");
    }

    info!("flume");

    let mut stream = client
        .flume(Input {
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
    let filter = EnvFilter::new("tonic_repo=info,client=info,server=info,hyper=trace,tokio=trace");

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
    type FuturesChannelStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type AsyncStreamStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type TokioMpscStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type UnboundedTokioMpscStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type AsyncChannelStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;
    type FlumeStream = Pin<Box<dyn Stream<Item = Result<Data, Status>> + Send>>;

    #[instrument(skip_all)]
    async fn futures_channel(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::FuturesChannelStream>, Status> {
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
        Ok(Response::new(Box::pin(rx) as Self::FuturesChannelStream))
    }

    #[instrument(skip_all)]
    async fn async_stream(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::AsyncStreamStream>, Status> {
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

        Ok(Response::new(Box::pin(output) as Self::AsyncStreamStream))
    }

    #[instrument(skip_all)]
    async fn tokio_mpsc(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::TokioMpscStream>, Status> {
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
        Ok(Response::new(Box::pin(rx) as Self::TokioMpscStream))
    }

    #[instrument(skip_all)]
    async fn unbounded_tokio_mpsc(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::UnboundedTokioMpscStream>, Status> {
        let request = request.into_inner();

        let (tx, rx) = tokio_unbounded_channel();

        let _sender = tokio::spawn(async move {
            for _ in 0..request.packets {
                let mut new_message = Data {
                    values: Vec::with_capacity(request.packet_size as usize),
                };
                for _ in 0..request.packet_size {
                    new_message.values.push(fastrand::i32(..));
                }
                info!("Sending message");
                if tx.send(Ok(new_message)).is_err() {
                    break;
                }
            }
        });
        let rx = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(rx) as Self::TokioMpscStream))
    }

    #[instrument(skip_all)]
    async fn async_channel(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::AsyncChannelStream>, Status> {
        let request = request.into_inner();

        let (tx, rx) = async_channel::bounded(10);

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
        Ok(Response::new(Box::pin(rx) as Self::AsyncChannelStream))
    }

    #[instrument(skip_all)]
    async fn flume(
        &self,
        request: Request<Input>,
    ) -> Result<Response<Self::AsyncChannelStream>, Status> {
        let request = request.into_inner();

        let (tx, rx) = flume::bounded(10);

        let _sender = tokio::spawn(async move {
            for _ in 0..request.packets {
                let mut new_message = Data {
                    values: Vec::with_capacity(request.packet_size as usize),
                };
                for _ in 0..request.packet_size {
                    new_message.values.push(fastrand::i32(..));
                }
                info!("Sending message");
                if tx.send_async(Ok(new_message)).await.is_err() {
                    break;
                }
            }
        });
        Ok(Response::new(
            Box::pin(rx.into_stream()) as Self::AsyncChannelStream
        ))
    }
}
