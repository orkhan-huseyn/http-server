use super::http::{HttpMethod, HttpStatusCode, Request, Response};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(realpath) => {
                if realpath.starts_with(&self.public_path) {
                    fs::read_to_string(realpath).ok()
                } else {
                    println!("Directory traversal attack attempted on {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            HttpMethod::GET => match request.path() {
                "/" => Response::new(HttpStatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(HttpStatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(HttpStatusCode::Ok, Some(content)),
                    None => Response::new(HttpStatusCode::NotFound, None),
                },
            },
            _ => Response::new(HttpStatusCode::NotFound, None),
        }
    }
}
