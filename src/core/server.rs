use std::collections::HashMap;

use crate::http::{generate_routes, parse_request};
use crate::config::Config;
use crate::http::Route;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    routes: HashMap<String, Route>, // path -> response
}

impl Server {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", config.server.address, config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        let routes = generate_routes(&config.routes)?;

        Ok(Self {
            listener,
            routes,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let routes = self.routes.clone();

            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // client closed connection
                    Ok(n) => {
                        match parse_request(&buf[..n]) {
                            Ok((method, path)) => {
                                Self::handle_connection(stream, method, path, routes).await;
                            }
                            Err(e) => eprintln!("Parse error: {e}"),
                        }
                    }
                    Err(e) => eprintln!("Read error: {e}"),
                }
            });
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        method: String,
        path: String,
        routes: HashMap<String, Route>,
    ) {
        // пока что просто проверим — есть ли путь
        if let Some(route) = routes.get(&path) {
            let response = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
            let _ = stream.write_all(response).await;
        } else {
            let response = b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found";
            let _ = stream.write_all(response).await;
        }
    }
}
