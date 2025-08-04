use std::collections::HashMap;

use crate::http::parse_request;
use crate::config::Config;
use crate::http::RouteType;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Server {
    routing: HashMap<String, RouteType>, // path: body
}

impl Server {
    async fn handle_connction(stream: TcpStream, method: String, path: String) {
        
    }

    async fn accept_connections(listener: TcpListener) -> anyhow::Result<()> {
        loop {
            let (mut stream, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // client closed connection
                    Ok(n) => {
                        match parse_request(&buf[..n]) {
                            Ok((method, path)) => {
                                Self::handle_connction(stream, method, path);
                            }
                            Err(e) => eprintln!("Parse error: {e}"),
                        }
                    }
                    Err(e) => eprintln!("Read error: {e}"),
                }
            });
        }
    }

    pub async fn run(config: Config) -> anyhow::Result<()> {
        let addr = format!("{}:{}", config.server.address, config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        Self::accept_connections(listener).await
    }
}