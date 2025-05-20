use tonic::transport::Server;
use tonic_repo::*;

#[tokio::main]
async fn main() {
    setup_logging();

    tokio::spawn(async {
        let _service = Server::builder()
            .initial_stream_window_size(16 * 1024 * 1024)
            .initial_connection_window_size(16 * 1024 * 1024)
            .max_concurrent_streams(1000)
            .add_service(example::streamer_server::StreamerServer::new(StreamerImpl))
            .serve("[::]:18080".parse().unwrap())
            .await;
    });

    run_client_requests().await;
}
