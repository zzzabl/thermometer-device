use rand::Rng;
use std::env;
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    loop {
        let value = rand::thread_rng().gen::<f32>();
        if let Err(e) = socket
            .send_to(&value.to_be_bytes(), format!("{}:{}", args[1], args[2]))
            .await
        {
            println!("Ошибка отправки {}", e.to_string());
        } else {
            println!("Ok");
        }
        sleep(Duration::from_secs(3)).await;
    }
}
