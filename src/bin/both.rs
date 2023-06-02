use tonic::transport::Server;
use tonic_repo::*;

#[tokio::main]
async fn main() {
    setup_logging();

    tokio::spawn(async {
        let _service = Server::builder()
            .add_service(example::streamer_server::StreamerServer::new(StreamerImpl))
            .serve("[::]:18080".parse().unwrap())
            .await;
    });

    run_client_requests().await;
}
