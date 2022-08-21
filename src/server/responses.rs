use warp::{http::{Response, response::Builder, StatusCode}, Reply};


fn base() -> Builder {
  return Response::builder()
    .header("X-Powered-By", "ArtieFuzzz")
    .header("Cache-Control", "public, max-age=7776000");
}
/// Returns an OK response
pub fn success() -> impl Reply {
    return base().status(200).body("OK").unwrap()
}

/// Create a custom response
pub fn custom(message: String, status: StatusCode) -> impl Reply {
  return base().status(status).body(message).unwrap()
}
