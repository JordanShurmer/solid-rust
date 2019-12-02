mod container;
mod resource;

use crate::error::Error;
use hyper::{Body, Request, Response};
use log::debug;
use tokio::fs;

// handle a request for a resource or container
pub async fn handle(request: &Request<Body>) -> Result<Response<Body>, Error> {
    let http_resource = crate::http::Resource::from_request(request).await?;
    let fs_metadata = fs::metadata(http_resource.file_path).await?;

    if fs_metadata.is_dir() {
        // a directory, treat it as an ldp:Container;
        debug!("Got a directory. Responding with an ldp:container)");
        return container::handle(request).await;
    }

    return resource::handle(request).await;
}
