use futures::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::{debug, error, warn};

static NOT_FOUND_BODY: &[u8] = b"NOT FOUND";

pub fn static_server(req: Request<Body>) -> super::our::ResponseFuture {
    // TODO: Update to async/.await when it's ready
    let file_path = req.uri().path().trim_start_matches('/').to_string();
    debug!("ldp handling requeset {} {}", req.method(), req.uri().path());
    debug!("file path: {}", file_path);
    match req.method() {
        &Method::GET => Box::new(
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
        ),
        _ => Box::new(
            futures::future::lazy(|| {
            Ok(Response::builder()
                .status(StatusCode::NOT_IMPLEMENTED)
                .body(Body::empty())
                .unwrap())
            })
        ),
    }
}
