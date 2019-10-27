use hyper::{Body, Request};
use log::debug;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::prelude::*;

#[derive(Debug)]
pub enum Resource {
    RDFSource(PathBuf),
    NonRDF(PathBuf),
    NotFound,
}

impl Resource {

    pub fn from(request: & Request<Body>) -> Self {
        let file_path = PathBuf::from(request.uri().path().trim_start_matches('/'));
        // Try to open the file. 
        if file_path.is_file() {

            debug!("found a file {:?}", file_path);
            if let Some(extension) = file_path.extension() {
                match extension.to_str() {
                    Some("ttl") => return Self::RDFSource(file_path),
    
                    Some("jsonld") => return Self::RDFSource(file_path),
    
                    _ => return Self::NonRDF(file_path),
                }
            }
            return Self::NonRDF(file_path);

        } else {
            debug!("not a file {:?}", file_path);
            Self::NotFound
        }
    }

    pub async fn to_body(&mut self) -> Result<Body, std::io::Error> {
        match self {
            Self::RDFSource(path) => {
                let mut file =  File::open(path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }

            Self::NonRDF(path) => {
                let mut file =  File::open(path).await?;
                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                Ok(Body::from(contents))
            }

            Self::NotFound => {
                let not_found: &[u8] = b"NOT FOUND";
                Ok(not_found.into())
            }
        }
    }

    pub fn content_type(&self) -> Option<&str> {
        if let Self::NotFound = self {
            None // todo: application/octet-stream or whatever
        } else {
            Some("text/turle")
        }
    }
}