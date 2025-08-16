use core::str;
use std::backtrace;
use std::collections::HashMap;
use std::fmt::format;
use std::sync::Arc;

use crate::http::{generate_routes, get_status_code_name, parse_request};
use crate::config::Config;
use crate::http::{Route, RouteType};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    routes: Arc<HashMap<String, Route>>, // path -> response
}

impl Server {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let addr = format!("{}:{}", config.server.address, config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        let routes = Arc::from(generate_routes(&config.routes).await?);

        info!("Server runs on {}:{}", config.server.address, config.server.port);

        Ok(Self {
            listener,
            routes,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let routes = Arc::clone(&self.routes);

            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // client closed connection
                    Ok(n) => {
                        match parse_request(&buf[..n]) {
                            Ok((method, path)) => {
                                Self::handle_connection(stream, method, path, routes).await;
                            }
                            Err(e) => warn!("Parse error: {e}"),
                        }
                    }
                    Err(e) => warn!("Read error: {e}"),
                }
            });
        }
    }

    async fn handle_connection(
        mut stream: TcpStream,
        method: String,
        path: String,
        routes: Arc<HashMap<String, Route>>,
    ) {
        if let Ok(addr) = stream.peer_addr() {debug!("New connection {}", addr)}

        if let Some(route) = routes.get(&path) {
            match &route.route_type {
                RouteType::Response(code, body) => {
                    let mut headers = format!(
                        "HTTP/1.1 {code} {}\r\n\
                        Content-Type: text/plain\r\n",
                        get_status_code_name(*code)
                    );

                    if route.gzip {
                        headers.push_str("Content-Encoding: gzip\r\n");
                    }

                    headers.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));

                    let response = format!("{headers}{body}");
                    let _ = stream.write_all(response.as_bytes()).await;
                }
                RouteType::Body(body) => {
                    let mut headers = String::from(
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/plain\r\n",
                    );

                    if route.gzip {
                        headers.push_str("Content-Encoding: gzip\r\n");
                    }

                    headers.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));

                    let response = format!("{headers}{body}");
                    let _ = stream.write_all(response.as_bytes()).await;
                }
                RouteType::Proxy(url) => {
                    Self::handle_reverse_proxy(stream, &url, &method).await;
                }
            };
        } else {
            let response = b"HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found";
            let _ = stream.write_all(response).await;
        }
    }

    async fn handle_reverse_proxy(mut client_stream: TcpStream, backend_addr: &str, request: &str) {
        if let Ok(mut backend) = TcpStream::connect(backend_addr).await {
            let _ = backend.write_all(request.as_bytes()).await;

            let mut buf = vec![0; 4096];
            if let Ok(n) = backend.read(&mut buf).await {
                let _ = client_stream.write_all(&buf[..n]).await;
            }
        } else {
            let response = b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 11\r\n\r\nBad Gateway";
            let _ = client_stream.write_all(response).await;
        }
    }
}
