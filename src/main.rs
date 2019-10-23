mod ldp;
mod our;

use futures::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{debug, error, info, warn};
use std::io;


// CLI Option Parsing Stuff
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "solid-rust")]
struct CliOpts {
    #[structopt(short, long, default_value = "7070")]
    port: u32,
}

// *** *** ***
// ENTRY POINT
// *** *** ***
fn main() {
    pretty_env_logger::init();

    let cli_opts = CliOpts::from_args();
    debug!("{:?}", cli_opts);

    // localhost, on the given port
    let addr = format!("127.0.0.1:{}", cli_opts.port).parse().unwrap();

    // Setup the hyper server
    // everything is handled by
    // our one function for now
    let server = Server::bind(&addr)
        .serve(|| service_fn(static_server))
        .map_err(|e| error!("server error: {}", e));

    info!("Listening on http://{}", addr);

    hyper::rt::run(server);
}

// Handle a request by resolving the url path from the current directly
fn static_server(req: Request<Body>) -> our::ResponseFuture {
    ldp::static_server(req)
}
