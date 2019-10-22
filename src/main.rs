use futures::{future, Future};

use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{debug, error, info, trace, warn};

use std::io;

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8000".parse().unwrap();

    let server = Server::bind(&addr)
        .serve(|| service_fn(static_server))
        .map_err(|e| error!("server error: {}", e));

    info!("Listening on http://{}", addr);

    hyper::rt::run(server);
}

type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = io::Error> + Send>;

fn static_server(req: Request<Body>) -> ResponseFuture {
    let file_path = req.uri().path().trim_start_matches('/').to_string();
    debug!("file path to serve: {}", file_path);
    Box::new(
        tokio_fs::file::File::open(file_path)
            .and_then(|file| {
                let buf: Vec<u8> = Vec::new();
                tokio_io::io::read_to_end(file, buf)
                    .and_then(|item| Ok(Response::new(item.1.into())))
                    .or_else(|err| {
                        warn!("error reading file {}", err);
                        Ok(Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::empty())
                            .unwrap())
                    })
            })
            .or_else(|err| {
                debug!("no file found: {}", err);
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap())
            }),
    )
}
