use warp::reject;

#[derive(Debug)]
pub struct NoContentProvided;

#[derive(Debug)]
pub struct NoValue;

impl reject::Reject for NoContentProvided {}
impl reject::Reject for NoValue {}