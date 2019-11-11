use core::hash::{Hash, Hasher};
use hyper::{Body, Request};
use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use tokio::fs::{metadata};

pub enum Conditional {
    PreconditionFailed,
    Valid,
}

pub async fn conditional(request: &Request<Body>) -> Conditional {
    if let Some(their_value) = request.headers().get("If-Match") {
        //todo: refactor.. duplicating etag computation here and in ldp.resource
        let file_path = PathBuf::from(request.uri().path().trim_start_matches('/'));
        if let Ok(metadata) = metadata(file_path).await {
            if let Ok(modified) = metadata.modified() {
                let modified = httpdate::fmt_http_date(modified);
                let mut h = DefaultHasher::new();
                modified.hash(&mut h);
                let our_value = h.finish().to_string();
                if our_value != their_value.to_str().unwrap() {
                    return Conditional::PreconditionFailed;
                }
            }
        }
    }

    Conditional::Valid
}
