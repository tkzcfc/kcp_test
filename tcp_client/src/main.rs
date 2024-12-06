use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;

async fn run_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3100").await?;

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
async fn main() -> io::Result<()> {
    loop {
        if let Err(err) = run_client().await {
            println!("error: {}", err);
        }
    }
}
