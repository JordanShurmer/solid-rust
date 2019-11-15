mod conditional;

use crate::error::Error;
use crate::error::Kind::*;
use conditional::Conditional;
use core::hash::{Hash, Hasher};
use hyper::{Body, Method, Request, Response, StatusCode};
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

        let conditional = Conditional::new(request, &modified_time, &etag);
        let get_or_head = request.method() == &Method::GET || request.method() == &Method::HEAD;

        // Conditional HTTP Request checks
        //  (return Err if it fails indicating why in the `kind` of error)
        match conditional.if_match() {
            Some(does_match) => {
                if !does_match {
                    return Err(Error {
                        kind: PreconditionFailed,
                    });
                }
            }

            // Only check If-Unmodified-Since when there was no If-Match given (per the spec)
            None => {
                if let Some(fresh) = conditional.if_unmodified_since() {
                    if !fresh {
                        return Err(Error {
                            kind: PreconditionFailed,
                        });
                    }
                }
            }
        }

        match conditional.if_none_match() {
            Some(none_match) => {
                if !none_match {
                    if get_or_head {
                        return Err(Error { kind: NotModified });
                    } else {
                        return Err(Error {
                            kind: PreconditionFailed,
                        });
                    }
                }
            }

            // Only check If-Modified-Since when there was no If-None-Match given (per the spec)
            None => {
                if get_or_head {
                    if let Some(newer) = conditional.if_modified_since() {
                        if !newer {
                            return Err(Error {
                                kind: NotModified,
                            });
                        }
                    }
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
