use core::convert::TryFrom;
use core::hash::{Hash, Hasher};
use hyper::{Body, Request};
use log::debug;
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use tokio::fs::{metadata, File};
use tokio::prelude::*;

#[derive(Debug)]
pub enum ResourceType {
    RDFSource,
    NonRDF,
}

#[derive(Debug)]
pub enum ResourceError {
    NotFound,
}

#[derive(Debug)]
pub struct Resource {
    resource_type: ResourceType,
    path: PathBuf,
}

impl TryFrom<&Request<Body>> for Resource {
    type Error = ResourceError;

    fn try_from(request: &Request<Body>) -> Result<Self, Self::Error> {
        let file_path = PathBuf::from(request.uri().path().trim_start_matches('/'));

        if file_path.is_file() {
            debug!("found a file {:?}", file_path);
            if let Some(extension) = file_path.extension() {
                match extension.to_str() {
                    Some("ttl") | Some("jsonld") => {
                        return Ok(Self {
                            resource_type: ResourceType::RDFSource,
                            path: file_path,
                        })
                    }

                    _ => {
                        return Ok(Self {
                            resource_type: ResourceType::NonRDF,
                            path: file_path,
                        })
                    }
                }
            }

            return Ok(Self {
                resource_type: ResourceType::NonRDF,
                path: file_path,
            });
        }

        return Err(ResourceError::NotFound);
    }
}

impl Resource {
    pub async fn to_body(&mut self) -> Result<Body, std::io::Error> {
        match self.resource_type {
            ResourceType::RDFSource => {
                let mut file = File::open(&self.path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }

            ResourceType::NonRDF => {
                let mut file = File::open(&self.path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }
        }
    }

    pub fn content_type(&self) -> &str {
        match self.resource_type {
            ResourceType::RDFSource => {
                if let Some(extension) = self.path.extension() {
                    match extension.to_str() {
                        Some("ttl") => return "text/turtle",
                        Some("jsonld") => return "application/ld+json",
                        _ => return "application/octet-stream", //octet stream?
                    }
                }
                "application/octet-stream" //octet stream?
            }

            ResourceType::NonRDF => "application/octet-stream", //octet stream?
        }
    }

    pub async fn last_modified(&self) -> String {
        // TODO cache the result - only lookup one time
        if let Ok(metadata) = metadata(&self.path).await {
            if let Ok(modified) = metadata.modified() {
                return httpdate::fmt_http_date(modified);
            }
        }
        "".to_owned()
    }

    pub async fn etag(&self) -> String {
        let modified = self.last_modified().await;
        let mut h = DefaultHasher::new();
        modified.hash(&mut h);
        return h.finish().to_string();
    }

    pub fn link(&self) -> &str {
        match &self.resource_type {
            ResourceType::RDFSource => "<http://www.w3.org/ns/ldp#RDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\"",

            ResourceType::NonRDF => "<http://www.w3.org/ns/ldp#NonRDFSource>; rel=\"type\", <http://www.w3.org/ns/ldp#Resource>; rel=\"type\""
        }
    }
}
