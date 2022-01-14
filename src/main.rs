use rand::Rng;
use std::net::UdpSocket;
use std::time::Duration;
use std::{env, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    loop {
        let value = rand::thread_rng().gen::<f32>();
        if let Err(e) = socket.send_to(&value.to_be_bytes(), format!("{}:{}", args[1], args[2])) {
            println!("Ошибка отправки {}", e.to_string());
        } else {
            println!("Ok");
        }
        thread::sleep(Duration::new(3, 0));
    }
}
