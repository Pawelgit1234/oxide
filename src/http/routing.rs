use std::collections::HashMap;
use std::fs;

use walkdir::WalkDir;

use crate::config::RouteConfig;

pub enum RouteType {
    Body(String), // body
    Proxy(String), // url
    Response(u16, String) // status code, body
}
