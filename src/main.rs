use futures::Future;

use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::{debug, error, info, warn};

use std::io;

static NOT_FOUND_BODY: &[u8] = b"NOT FOUND";

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


// Some short hand for dyn+send box
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = io::Error> + Send>;

// Handle a request by resolving the url path from the current directly
fn static_server(req: Request<Body>) -> ResponseFuture {
    // TODO: Update to async/.await when it's ready
    let file_path = req.uri().path().trim_start_matches('/').to_string();
    debug!("file path to serve: {}", file_path);
    Box::new(
        tokio_fs::file::File::open(file_path)
            .and_then(|file| {
                let buf: Vec<u8> = Vec::new();
                // TODO: stream the file instead
                tokio_io::io::read_to_end(file, buf)
                    .and_then(|item| Ok(Response::new(item.1.into())))
                    .or_else(|err| {
                        error!("error reading file {}", err);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap())
                    })
            })
            .or_else(|err| {
                warn!("no file found: {}", err);
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(NOT_FOUND_BODY.into())
                    .unwrap())
            }),
    )
}
