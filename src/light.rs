use change_light::Light;

#[tokio::main]
async fn main() {
    let bind_addr = ([127,0,0,1], 2000).into();
    let connect_addr = ([127,0,0,1], 1883).into();
    let mut client = mquictt_client::Client::connect(
        &bind_addr,
        &connect_addr,
        "localhost",
        "0",
        mquictt_client::Config::read(&"certs/light.json").unwrap()
    ).await.unwrap();
    let mut sub = client.subscriber("bedroom/light").await.unwrap();

    while let Ok(data) = sub.read().await {
        let settings: Light = serde_json::from_slice(&data).unwrap();
        println!("Changing light's settings to: {:#?}", settings);
    }
}
