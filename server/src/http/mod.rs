use crate::error::Error;
use crate::error::Kind::*;
use core::hash::{Hash, Hasher};
use hyper::{Body, Request, Response, StatusCode};
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use tokio::fs::metadata;

// A representation of an HTTP Resource within this crate
#[derive(Debug)]
pub struct Resource {
    pub file_path: PathBuf,
    pub last_modified: String,
    pub etag: String,
}

impl Resource {

    // create a resource from an http request
    pub async fn from_request(request: &Request<Body>) -> Result<Self, Error> {
        let file_path = PathBuf::from(request.uri().path().trim_start_matches('/'));
        let modified_time = metadata(file_path.clone()).await?.modified()?;
        let last_modified = httpdate::fmt_http_date(modified_time);

        // ETag value is a hash of the last modified time for now
        let mut h = DefaultHasher::new();
        last_modified.hash(&mut h);
        let etag = h.finish().to_string();

        // Conditional HTTP Request checks
        //  (return Err if it fails indicating why in the `kind` of error)
        if let Some(request_header) = request.headers().get("If-Match") {
            if etag != request_header.to_str().unwrap_or_default() {
                return Err(Error {
                    kind: PreconditionFailed,
                });
            }
        }

        if let Some(request_header) = request.headers().get("If-Unmodified-Since") {
            if let Ok(header_value) = httpdate::parse_http_date(request_header.to_str().unwrap_or_default()) {
                if modified_time > header_value {
                    return Err(Error {
                        kind: PreconditionFailed,
                    })
                }
            }
        }
        
        if let Some(request_header) = request.headers().get("If-None-Match") {
            for header_tag in request_header.to_str().unwrap_or_default().split(",") {
                if etag == header_tag.trim() {
                    return Err(Error {
                        kind: NotModified
                    });
                }
            }
        }

        Ok(Self {
            file_path,
            etag,
            last_modified,
        })
    }

    // get an http response builder with general http specific stuff filled in
    pub fn response_builder(&self) -> http::response::Builder {
        let mut builder = Response::builder();
        builder
            .status(StatusCode::OK)
            .header("Last-Modified", &self.last_modified)
            .header("ETag", &self.etag);

        builder
    }
}
