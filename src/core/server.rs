use crate::http::parse_request;
use crate::config::Config;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Server {

}

impl Server {
    pub async fn run(config: Config) -> anyhow::Result<()> {
        let addr = format!("{}:{}", config.server.address, config.server.port);
        let listener = TcpListener::bind(&addr).await?;

        loop {
            let (mut stream, addr) = listener.accept().await?;
            
            tokio::spawn(async move {
                let mut buf = [0u8; 4096]; 
                match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // client closed connection
                    Ok(n) => {
                        match parse_request(&buf[..n]) {
                            Ok((method, path)) => {
                                let response = b"HTTP/1.1 200 OK\r\nContent-Lenght: 2\r\n\r\nOK";
                                let _ = stream.write_all(response).await;
                            }
                            Err(e) => eprintln!("Parse error {e}"),
                        }
                    }
                    Err(e) => eprintln!("Read error: {e}"),
                }
            });
        }
    }
}