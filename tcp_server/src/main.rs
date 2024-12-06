use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];

    loop {
        // 读取数据
        let n = match socket.read(&mut buf).await {
            Ok(0) => return Ok(()), // 客户端关闭连接
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        // 回显数据
        if socket.write_all(&buf[..n]).await.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to send data"));
        }
    }
}

async fn run_server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_client(socket)); // 为每个客户端启动一个新的任务
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    run_server("0.0.0.0:3100").await
}
