// *** *** ***
// A place to store shared resuable things
// constants, types, etc.
//
// can be acceessed with
//   mod our;
//   our::Whatever
use hyper::{Body, Response};

// Some short hand for dyn+send box
pub type ServerResult = Result<Response<Body>, Box<dyn std::error::Error>>;