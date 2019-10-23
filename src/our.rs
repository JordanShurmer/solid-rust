// *** *** ***
// A place to store shared resuable things
// constants, types, etc.
//
// can be acceessed with
//   mod our;
//   our::Whatever
use futures::Future;
use hyper::{Body, Response};
use std::io;

// Some short hand for dyn+send box
pub type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = io::Error> + Send>;