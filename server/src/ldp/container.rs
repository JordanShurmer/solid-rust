use crate::error::Error;
use crate::http::Resource;
use hyper::{Body, Request, Response, StatusCode};
use log::debug;

pub async fn handle(request: &Request<Body>) -> Result<Response<Body>, Error> {
    if !request.uri().path().ends_with("/") {
        // redirect to the same path with / appended
        debug!("Adding '/' to end of container url");
        return Ok(Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("Location", format!("{}/",  request.uri()))
            .body(Body::empty())?
        );
    }

    let container = Container{
        http_resource: Resource::from_request(request).await?,
    };

    let mut builder = container.response_builder();

    Ok(builder.body(Body::empty())?)

}

#[derive(Debug)]
pub struct Container {
    http_resource: Resource,
}

impl Container {
    pub fn response_builder(&self)  -> http::response::Builder {
        let mut builder: hyper::http::response::Builder = self.http_resource.response_builder();

        // Add the LDP specific metadata
        builder
            .header("Content-Type", "text/turtle")
            .header("Link", "<http://www.w3.org/ns/ldp#BasicContainer>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"");

        builder
    }
}