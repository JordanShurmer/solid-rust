mod resource;

use hyper::{Body, Method, Request, Response, StatusCode};
use log::debug;
use resource::Resource;

pub async fn handle(request: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    debug!(
        "ldp handling request {} {}",
        request.method(),
        request.uri().path()
    );

    match request.method() {
        &Method::GET => {
            let mut resource = Resource::from(&request);

            if let Resource::NotFound = resource {
                let not_found: &[u8] = b"NOT FOUND";
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(not_found.into())
                    .unwrap())
            } else {
                Ok(rdf_response(resource.content_type())
                    .status(StatusCode::OK)
                    .header("ETag", resource.etag().await)
                    .body(resource.to_body().await?)
                    .unwrap())
            }
        }

        &Method::HEAD => {
            let resource = Resource::from(&request);

            if let Resource::NotFound = resource {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap())
            } else {
                Ok(rdf_response(resource.content_type())
                    .status(StatusCode::OK)
                    .header("ETag", resource.etag().await)
                    .body(Body::empty())
                    .unwrap())
            }
        }

        &Method::OPTIONS => Ok(rdf_response(None).body(Body::empty()).unwrap()),

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
        .header("Allow", "GET,HEAD,OPTIONS");

    return builder;
}
