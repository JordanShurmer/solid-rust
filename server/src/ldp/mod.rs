mod resource;

use core::convert::TryFrom;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::debug;
use resource::{Resource, ResourceError};

pub async fn handle(request: Request<Body>) -> crate::our::ServerResult {
    debug!(
        "ldp handling request {} {}",
        request.method(),
        request.uri().path()
    );

    match request.method() {
        &Method::GET => match Resource::try_from(&request) {
            Ok(mut resource) => Ok(rdf_response(resource.content_type(), resource.link())
                .status(StatusCode::OK)
                .header("Last-Modified", resource.last_modified().await)
                .header("ETag", resource.etag().await)
                .body(resource.to_body().await?)
                .unwrap()),

            Err(e) => match e {
                ResourceError::NotFound => {
                    let not_found: &[u8] = b"NOT FOUND";
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(not_found.into())
                        .unwrap())
                }
            },
        },

        &Method::HEAD => match Resource::try_from(&request) {
            Ok(resource) => Ok(rdf_response(resource.content_type(), resource.link())
                .status(StatusCode::OK)
                .header("Last-Modified", resource.last_modified().await)
                .header("ETag", resource.etag().await)
                .body(Body::empty())
                .unwrap()),

            Err(e) => match e {
                ResourceError::NotFound => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap()),
            },
        },

        &Method::OPTIONS => Ok(base_response().body(Body::empty()).unwrap()),

        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Accept", "GET")
            .body(Body::empty())
            .unwrap()),
    }
}

// *** *** ***
// setup the response with the
// RDF Resource related info
// *** *** ***
fn base_response() -> http::response::Builder {
    let mut builder = Response::builder();
    builder.header("Allow", "GET,HEAD,OPTIONS");

    return builder;
}
fn rdf_response(content_type: &str, link: &str) -> http::response::Builder {
    let mut builder = base_response();
    builder
        .header("Content-Type", content_type)
        .header("Link", link);
    builder
}
