use crate::http::media_type::MediaType;
use crate::error::{Error, Kind};
use hyper::{Body, Request};
use tokio::fs::File;
use tokio::prelude::*;

#[derive(Debug)]
pub enum ResourceType {
    RDFSource,
    NonRDF,
}

// An LDP Resource is an HTTP Resource with a bit of metadata added
#[derive(Debug)]
pub struct Resource {
    http_resource: crate::http::Resource,
    content_type: String,
    resource_type: ResourceType,
}

impl Resource {
    pub async fn from_request(request: &Request<Body>) -> Result<Self, Error> {
        // get an http resource and turn that `into` our resource
        Ok(crate::http::Resource::from_request(&request).await?.into())
    }

    // Turn the resource into an http body
    pub async fn http_body(&mut self, desired_media_type: Option<&str>) -> Result<Body, Error> {
        match self.resource_type {
            ResourceType::RDFSource => {
                let desired_media_type = desired_media_type.unwrap_or("text/tutle");
                let our_media_type: MediaType = self.content_type.as_str().into();

                // TODO: load RDF and translate between content types
                if our_media_type.matches(desired_media_type) {
                    let mut file = File::open(&self.http_resource.file_path).await?;
                    let mut contents = vec![];
                    file.read_to_end(&mut contents).await?;
                    return Ok(Body::from(contents));
                }
                Err(Error {
                    kind: Kind::NotAcceptable,
                })
            }

            ResourceType::NonRDF => {
                let mut file = File::open(&self.http_resource.file_path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }
        }
    }

    // Turn the resource into an http response builder
    pub fn response_builder(&self) -> http::response::Builder {
        let link = match self.resource_type {
            ResourceType::RDFSource => "<http://www.w3.org/ns/ldp#RDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"",

            ResourceType::NonRDF => "<http://www.w3.org/ns/ldp#NonRDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\""
        };

        let mut builder: hyper::http::response::Builder = self.http_resource.response_builder();

        // Add the LDP specific metadata
        builder
            .header("Content-Type", &self.content_type)
            .header("Link", link);

        builder
    }
}

impl From<crate::http::Resource> for Resource {
    // Turn an http resource into an ldp resource
    fn from(resource: crate::http::Resource) -> Self {
        // Derive content types from the file extension :\?
        // this will change when we support Content Negotiation
        let extension = resource.file_path.extension().unwrap_or_default();
        let (resource_type, content_type) = match extension.to_str() {
            Some("ttl") => (ResourceType::RDFSource, "text/turtle".to_owned()),
            Some("jsonld") => (ResourceType::RDFSource, "application/ld+json".to_owned()),
            _ => (ResourceType::NonRDF, "application/octet-stream".to_owned()),
        };

        Self {
            http_resource: resource,
            resource_type,
            content_type,
        }
    }
}