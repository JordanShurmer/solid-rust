use std::time::SystemTime;

pub struct Conditional<'a> {
    if_match_header: Option<String>,
    if_unmodified_header: Option<SystemTime>,
    
    if_none_match_header: Option<String>,
    if_modified_header: Option<SystemTime>,

    last_modified: &'a SystemTime,
    etag: &'a str,
}

fn string_header(request: &hyper::Request<hyper::Body>, header: &str) -> Option<String> {
    request.headers()
        .get(header)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_owned())
}

fn date_header(request: &hyper::Request<hyper::Body>, header: &str) -> Option<SystemTime> {
    request.headers()
        .get(header)
        .and_then(|value| value.to_str().ok())
        .and_then(|s| httpdate::parse_http_date(s).ok())
}

impl Conditional<'_> {

    pub fn new<'a>(request: &hyper::Request<hyper::Body>, last_modified: &'a SystemTime, etag: &'a String) ->  Conditional<'a> {

        Conditional {
            if_match_header: string_header(request, "If-Match"),
            if_unmodified_header: date_header(request, "If-Unmodified-Since"),
            if_none_match_header: string_header(request, "If-None-Match"),
            if_modified_header: date_header(request, "If-Modified-Since"),

            last_modified,
            etag,
        }
    }

    pub fn if_match(&self) -> Option<bool> {
        self.if_match_header.as_ref()
            .map(|header| self.etag == header)
    }

    pub fn if_unmodified_since(&self) -> Option<bool> {
        self.if_unmodified_header.as_ref()
            .map(|header| self.last_modified <= header)
    }

    pub fn if_none_match(&self) -> Option<bool> {
        self.if_none_match_header.as_ref()
        .map(|header| !header.split(",").any(|value| self.etag == value.trim()))
    }

    pub fn if_modified_since(&self) -> Option<bool> {
        self.if_modified_header.as_ref()
        .map(|header| self.last_modified > header)
    }
}