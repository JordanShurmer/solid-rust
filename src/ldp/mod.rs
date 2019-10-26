use hyper::{Body, Method, Request, Response, StatusCode};
use log::{debug, error, info, warn};
use std::path::Path;
use tokio::fs::File;
use tokio::prelude::*;

static NOT_FOUND_BODY: &[u8] = b"NOT FOUND";

#[derive(Debug)]
enum Resource {
    RDFSource(File),
    NonRDF(File),
}

impl Resource {
    async fn from(request: Request<Body>) -> Result<Self, std::io::Error> {
        let file_path = Path::new(request.uri().path().trim_start_matches('/'));
        if let Some(extension) = file_path.extension() {
            match extension.to_str() {
                Some("ttl") => return Ok(Self::RDFSource(File::open(file_path).await?)),

                Some("jsonld") => return Ok(Self::RDFSource(File::open(file_path).await?)),

                _ => return Ok(Self::NonRDF(File::open(file_path).await?)),
            }
        }

        return Ok(Self::NonRDF(File::open(file_path).await?));
    }

    async fn to_body(&mut self) -> Result<Body, std::io::Error> {
        match self {
            Self::RDFSource(file) => {
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }

            Self::NonRDF(file) => {
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }
        }
    }
}

pub async fn handle(request: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    debug!(
        "ldp handling request {} {}",
        request.method(),
        request.uri().path()
    );

    // Handle each method properly
    match request.method() {
        // *** ***
        // GET requests
        // *** ***
        &Method::GET => {
            let file_path = request.uri().path().trim_start_matches('/').to_string();
            let path = Path::new(&file_path);
            debug!("file path: {:?}", path);

            // if non-rs: read and return file with headers
            // if RDFSource:
            //   inspect Accept headers
            //   adapt to the right type
            //   return adapted contents with headers

            match Resource::from(request).await {

                Ok(mut resource) => Ok(rdf_response(Some("text/turtle"))
                        .status(StatusCode::OK)
                        .body(resource.to_body().await?)
                        .unwrap()),

                Err(e) => {
                    debug!("error {}", e);
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(NOT_FOUND_BODY.into())
                        .unwrap())
                }
            }
        }

        &Method::HEAD => Ok(rdf_response(Some("text/turtle"))
            .body(Body::empty())
            .unwrap()),

        &Method::OPTIONS => Ok(rdf_response(None).body(Body::empty()).unwrap()),

        // *** ***
        // Unimplemented requests
        // *** ***
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
fn rdf_response(content_type: Option<&str>) -> http::response::Builder {
    let mut builder = Response::builder();
    if let Some(type_header) = content_type {
        builder.header("Content-Type", type_header);
    }

    builder
        .header("Link", "<http://www.w3.org/ns/ldp#RDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"")
        .header("Accept", "GET,OPTIONS");

    return builder;
}
