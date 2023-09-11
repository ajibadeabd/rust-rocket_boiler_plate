use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, ContentType, Status};
use std::fs::OpenOptions;
use std::io::{Write, Result};

pub struct RequestLogger {
    pub get: AtomicUsize,
    pub post: AtomicUsize,
    pub put: AtomicUsize,
    pub delete: AtomicUsize,
    pub patch: AtomicUsize,
}

impl RequestLogger {
    fn log_request(&self, request: &Request) -> Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("request_log.txt")?;

        let method = request.method();
        let path = request.uri().path();
        let log_entry = format!("Method: {:?}, Path: {}\n", method, path);

        file.write_all(log_entry.as_bytes())?;
        Ok(())
    }
}

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            Method::Patch => self.patch.fetch_add(1, Ordering::Relaxed),
            Method::Put => self.put.fetch_add(1, Ordering::Relaxed),
            Method::Delete => self.delete.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };

        if let Err(err) = self.log_request(request) {
            eprintln!("Failed to log request: {:?}", err);
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, _: &mut Response<'r>) {
        // Do nothing on response.
    }
}
