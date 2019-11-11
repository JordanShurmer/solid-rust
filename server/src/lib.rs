mod ldp;
mod our;
mod base_http;

use base_http::Conditional;
use hyper::error::Error;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{error, info};
use tokio::sync::oneshot;

// *** *** ***
// ENTRY POINT
// *** *** ***
// pub async fn serve(port: u16, stop: oneshot::Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
pub async fn serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // pretty_env_logger::init();

    // localhost, on the given port
    let addr = ([127, 0, 0, 1], port).into();

    info!("starting server {:?}", addr);
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

    // let graceful = server.with_graceful_shutdown(async {
        // stop.await.ok();
    // });

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}

// Take a request, and dispatch it to the right server
async fn dispatch(request: Request<Body>) -> our::ServerResult {

    match base_http::conditional(&request).await {

        Conditional::PreconditionFailed => Ok (
            Response::builder()
            .status(StatusCode::PRECONDITION_FAILED)
            .body(hyper::Body::empty())
            .unwrap()
        ),

        Conditional::Valid => ldp::handle(request).await
    }

}
