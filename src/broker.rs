#[tokio::main]
async fn main() {
    mquictt_server::server(
        &([127, 0, 0, 1], 1883).into(),
        mquictt_server::Config::read(&"certs/broker.json").unwrap(),
    )
    .await
    .unwrap();
}