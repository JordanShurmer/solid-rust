mod ldp;
mod our;

use hyper::error::Error;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{error, info};

// *** *** ***
// ENTRY POINT
// *** *** ***
pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {

    // localhost, on the given port
    let addr = ([127, 0, 0, 1], port).into();

    // Setup the hyper server everything is handled by our one function for now
    let server = Server::bind(&addr).serve(make_service_fn(|_: &AddrStream| {
        // return a service_function that handles a request
        async move {
            Ok::<_, Error>(service_fn(move |request: Request<Body>| {
                async {
                    // call the handler, we handle errors with a 500 response
                    match dispatch(request).await {
                        Ok(response) => Ok::<_, Error>(response),

                        Err(e) => {
                            error!("Error reading file, {:?}", e);
                            Ok::<_, Error>(
                                Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Body::empty())
                                    .unwrap(),
                            )
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
async fn dispatch(request: Request<Body>) -> our::ServerResult {
    // LDP server is the only one right now
    ldp::handle(request).await
}
