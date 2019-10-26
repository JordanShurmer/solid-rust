mod ldp;
// mod our;

use hyper::server::conn::AddrStream;
use hyper::service::service_fn;
use hyper::service::make_service_fn;
use hyper::{Body, Request, Server};
use hyper::error::Error;
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
    let server = Server::bind(&addr)
        .serve(make_service_fn(|_: &AddrStream| {
            // return a service_function that handles a request
            async move {
                Ok::<_, Error>(service_fn(move |request: Request<Body>| async {
                    Ok::<_, Error>(
                        ldp::handle(request).await
                    )
                }))
            }
        }));

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    } 

    Ok(())
}
