use std::net::SocketAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;
use tokio_kcp::{KcpConfig, KcpStream};

fn now_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

async fn run_client() -> anyhow::Result<()> {
    println!("run_client===========================>>");
    let config = KcpConfig::default();

    let server_addr = "127.0.0.1:3100".parse::<SocketAddr>()?;

    let mut stream = KcpStream::connect(&config, server_addr).await?;

    let message = [0u8;65535];
    loop {
        stream.write_all(&message).await?;

        let mut buffer = BytesMut::with_capacity(65535);
        stream.read_buf(&mut buffer).await?;
        println!("recv len: {}", buffer.len());

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    loop {
        if let Err(err) = run_client().await {
            println!("error: {}", err);
        }
    }
}
