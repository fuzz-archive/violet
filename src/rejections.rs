use warp::reject;

#[derive(Debug)]
pub struct NoContentProvided;

impl reject::Reject for NoContentProvided {}

#[derive(Debug)]
pub struct NoValue;

impl reject::Reject for NoValue {}