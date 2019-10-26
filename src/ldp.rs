use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::fs::File;
use tokio::prelude::*;
use log::{debug, error, info, warn};
use std::path::Path;

static NOT_FOUND_BODY: &[u8] = b"NOT FOUND";

pub async fn handle(request: Request<Body>) -> Response<Body> {
    let file_path = request.uri().path().trim_start_matches('/').to_string();
    debug!(
        "ldp handling requeset {} {}",
        request.method(),
        request.uri().path()
    );
    debug!("file path: {}", file_path);

    // Handle each method properly
    match request.method() {
        // *** ***
        // GET requests
        // *** ***
        &Method::GET => {
            let path = Path::new(&file_path);

            match File::open(path).await {

                Ok(mut file) => {
                    let mut contents = vec![];
                    file.read_to_end(&mut contents).await.unwrap();
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "text/turtle")
                        .header("Link", "<http://www.w3.org/ns/ldp#RDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"")
                        .header("Accept", "GET,OPTIONS")
                        .body(Body::from(contents))
                        .unwrap()

                },

                Err(e) => {
                    error!("error {}", e);
                    Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(NOT_FOUND_BODY.into())
                    .unwrap()
                }
            }
        },

        // *** ***
        // Unimplemented requests
        // *** ***
        _ => Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .header("Accept", "GET")
                .body(Body::empty())
                .unwrap()
    }
}

// *** *** ***
// setup the response with the
// RDF Resource related info
// *** *** ***
fn rdf_response<'a>(builder: &'a mut http::response::Builder) -> &'a mut http::response::Builder {
    builder
        .header("Link", "<http://www.w3.org/ns/ldp#RDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"")
        .header("Accept", "GET,OPTIONS")
}