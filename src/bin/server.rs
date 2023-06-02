use tonic::transport::Server;
use tonic_repo::example;
use tonic_repo::setup_logging;
use tonic_repo::StreamerImpl;

#[tokio::main]
async fn main() {
    setup_logging();
    let _service = Server::builder()
        .add_service(example::streamer_server::StreamerServer::new(StreamerImpl))
        .serve("[::]:18080".parse().unwrap())
        .await;
}
