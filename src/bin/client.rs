use tonic_repo::*;

#[tokio::main]
async fn main() {
    setup_logging();
    run_client_requests().await;
}
