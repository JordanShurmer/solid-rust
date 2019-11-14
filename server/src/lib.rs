mod error;
mod http;
mod ldp;

use error::Kind::*;
use hyper::error::Error;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{debug, error, info};

// *** *** ***
// ENTRY POINT
// *** *** ***
pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // localhost, on the given port
    let addr = ([127, 0, 0, 1], port).into();

    info!("starting server {:?}", addr);

    // Setup the hyper server everything is handled by our one function for now
    let server = Server::bind(&addr).serve(make_service_fn(|_: &AddrStream| {
        // return a service_function that handles a single request
        async move {
            Ok::<_, Error>(service_fn(move |request: Request<Body>| {
                async {
                    match dispatch(request).await {
                        // Ok Results need no server level additions
                        Ok(response) => Ok::<_, Error>(response),

                        // Err Results are handled here
                        Err(e) => {
                            debug!("Error reading file, {:?}", e);
                            match e.kind {
                                // 404
                                NotFound => {
                                    let not_found: &[u8] = b"NOT FOUND";
                                    Ok::<_, Error>(
                                        Response::builder()
                                            .status(StatusCode::NOT_FOUND)
                                            .body(not_found.into())
                                            .unwrap(),
                                    )
                                }

                                // 412
                                PreconditionFailed => Ok(Response::builder()
                                    .status(StatusCode::PRECONDITION_FAILED)
                                    .body(hyper::Body::empty())
                                    .unwrap()),

                                // 405
                                MethodNotAllowed => Ok(Response::builder()
                                    .status(StatusCode::METHOD_NOT_ALLOWED)
                                    .header("Allow", "GET,HEAD,OPTIONS")
                                    .body(Body::empty()).unwrap()),

                                // 304
                                NotModified => Ok(Response::builder()
                                    .status(StatusCode::NOT_MODIFIED)
                                    .body(Body::empty()).unwrap()),

                                // 500
                                _ => Ok::<_, Error>(
                                    Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Body::empty())
                                        .unwrap(),
                                ),
                            }
                        }
                    }
                }
            }))
        }
    }));

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}

// Take a request, and dispatch it to the right server
async fn dispatch(request: Request<Body>) -> Result<Response<Body>, error::Error> {

    // handle OPTIONS requests immediately
    if &Method::OPTIONS == request.method() {
        return Ok(Response::builder()
            .header("Allow", "GET,HEAD,OPTIONS")
            .body(Body::empty())?);
    }

    // currently everything is an LDP resource, but this is where
    // the different types of resources will be differentiated
    let mut resource = ldp::Resource::from_request(&request).await?;
    debug!("ldp resource: {:?}", resource);

    // Get a response builder, then finish building the response
    let mut response: hyper::http::response::Builder = resource.response_builder();
    response.header("Allow", "GET,HEAD,OPTIONS");
    match request.method() {
        &Method::GET => Ok(response.body(resource.http_body().await?)?),

        &Method::HEAD => Ok(response.body(Body::empty())?),

        _ => Err(error::Error{ kind: MethodNotAllowed })
    }
}
