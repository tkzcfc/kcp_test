use std::{net::SocketAddr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_kcp::{KcpConfig, KcpStream};

fn now_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let config = KcpConfig::default();

    let server_addr = "127.0.0.1:3100".parse::<SocketAddr>().unwrap();

    let mut stream = KcpStream::connect(&config, server_addr).await.unwrap();

    loop {
        let now = now_millis();
        stream.write_u128(now).await.unwrap();
        println!("ping is : {}", now - stream.read_u128().await.unwrap());
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
