use std::collections::HashMap;
use std::fs;

use walkdir::WalkDir;

use crate::config::RouteConfig;
use crate::http::gzip_response;

#[derive(Clone, Debug)]
pub struct Route {
    pub route_type: RouteType,
    pub gzip: bool,
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum RouteType {
    Body(Vec<u8>), // body
    Proxy(String), // url
    Response(u16, Vec<u8>) // status code, body
}

pub async fn generate_routes(route_configs: &Vec<RouteConfig>) -> anyhow::Result<HashMap<String, Route>> {
    let mut routes: HashMap<String, Route> = HashMap::new();

    for route_config in route_configs.iter() {
        let gzip = route_config.gzip.unwrap_or(false);
        let timeout_ms = route_config.timeout_ms;

        if let Some(index_path) = &route_config.index {
            let mut body = std::fs::read(index_path)?;
            if gzip { body = gzip_response(&body).await?; }


            let route = Route {
                route_type: RouteType::Body(body),
                gzip: gzip,
                timeout_ms: timeout_ms
            };
            routes.insert(route_config.path.clone(), route);
        } else if let Some(directory_path) = &route_config.directory {
            for file in WalkDir::new(directory_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let mut body = fs::read(file.path())?;
                if gzip { body = gzip_response(&body).await?; }

                let relative_path = file.path().strip_prefix(directory_path)?;
                let full_path = format!(
                    "{}/{}",
                    route_config.path.trim_end_matches('/'),
                    relative_path.to_string_lossy()
                );

                let route = Route {
                    route_type: RouteType::Body(body),
                    gzip,
                    timeout_ms,
                };

                routes.insert(full_path, route);
            }
        } else if let Some(proxy_pass) = &route_config.directory {
            let route = Route {
                route_type: RouteType::Proxy(proxy_pass.clone()),
                gzip: false,
                timeout_ms: timeout_ms,
            };
            routes.insert(route_config.path.clone(), route);
        } else if let Some(response) = &route_config.response {
            let mut body = response.body.clone().into_bytes();
            if gzip { body = gzip_response(&body).await?; }

            let route = Route {
                route_type: RouteType::Response(response.status, body),
                gzip,
                timeout_ms,
            };
            routes.insert(route_config.path.clone(), route);
        };
    }

    Ok(routes)
}