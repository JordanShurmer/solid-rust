mod resource;

use crate::error::Error;
use crate::error::Kind::*;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::debug;
use resource::Resource;
use tokio::fs;

// handle a request for a resource or container
pub async fn handle(request: &Request<Body>) -> Result<Response<Body>, Error> {
    let http_resource = crate::http::Resource::from_request(request).await?;
    let fs_metadata = fs::metadata(http_resource.file_path).await?;
    let host = &request.headers()["Host"];

    if fs_metadata.is_dir() {
        // a directory, treat it as an ldp:Container;
        debug!("Got a directory. Responding with an ldp:container)");
        if !request.uri().path().ends_with("/") {
            // redirect to the same path with / appended
            debug!("Adding '/' to end of container url");
            return Ok(Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header("Location", format!("{}/",  request.uri()))
                .body(Body::empty())?
            );
        }
    }


    //  not a directory, must be a file. Treat it as an ldp:Resource
    let mut resource = Resource::from_request(request).await?;
    debug!("ldp resource: {:?}", resource);

    // Get a response builder, then finish building the response
    let mut response: hyper::http::response::Builder = resource.response_builder();
    response.header("Allow", "GET,HEAD,OPTIONS");
    match request.method() {
        &Method::GET => Ok(response.body(
            resource
                .http_body(
                    request
                        .headers()
                        .get("Accept")
                        .and_then(|header| header.to_str().ok()),
                )
                .await?,
        )?),

        &Method::HEAD => Ok(response.body(Body::empty())?),

        _ => Err(Error {
            kind: MethodNotAllowed,
        }),
    }
}
