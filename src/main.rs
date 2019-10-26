mod ldp;
// mod our;

use hyper::error::Error;
use hyper::server::conn::AddrStream;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{debug, error, info};

// CLI Option Parsing Stuff
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "solid-rust")]
struct CliOpts {
    #[structopt(short, long, default_value = "7070")]
    port: u16,
}

// *** *** ***
// ENTRY POINT
// *** *** ***
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let cli_opts = CliOpts::from_args();
    debug!("{:?}", cli_opts);

    // localhost, on the given port
    let addr = ([127, 0, 0, 1], cli_opts.port).into();

    // Setup the hyper server
    // everything is handled by
    // our one function for now
    let server = Server::bind(&addr).serve(make_service_fn(|_: &AddrStream| {
        // return a service_function that handles a request
        async move {
            Ok::<_, Error>(service_fn(move |request: Request<Body>| {
                async {
                    // call the handler, we handle errors with a 500 response
                    match ldp::handle(request).await {
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
