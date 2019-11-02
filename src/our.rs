// *** *** ***
// A place to store shared resuable things
// constants, types, etc.
//
// can be acceessed with
//   mod our;
//   our::Whatever

// Some short hand for dyn+send box
pub type ServerResult = Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error>>;
