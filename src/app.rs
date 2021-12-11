use change_light::Light;

#[tokio::main]
async fn main() {
    let mut settings = Light::default();
    let bind_addr = ([127, 0, 0, 1], 2001).into();
    let connect_addr = ([127, 0, 0, 1], 1883).into();
    let mut client = mquictt_client::Client::connect(
        &bind_addr,
        &connect_addr,
        "localhost",
        "0",
        mquictt_client::Config::read(&"certs/app.json").unwrap(),
    )
    .await
    .unwrap();
    let mut pbl = client
        .publisher("bedroom/light", bytes::Bytes::from("hello"))
        .await
        .unwrap();

    loop {
        println!("1. Change Color\n2. Change Brightness\n> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        match input.trim_end().parse::<u8>().unwrap() {
            1 => {
                println!("1. Red\n2. Green\n3. Blue\n(others = White)> ");
                input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");
                settings.color = match input.trim_end().parse::<u8>().unwrap() {
                    1 => "Red".to_owned(),
                    2 => "Green".to_owned(),
                    3 => "Blue".to_owned(),
                    _ => String::new(), // White
                };
            }
            2 => {
                println!("Brightness value, out of 256 > ");
                input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");
                settings.brightness = input.trim_end().parse::<u8>().unwrap();
            }
            _ => {
                println!("Unsupported Operation");
            }
        }

        pbl.publish(bytes::Bytes::from(serde_json::to_vec(&settings).unwrap()))
            .unwrap();
        pbl.flush().await.unwrap();
    }
}
