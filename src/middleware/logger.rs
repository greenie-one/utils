use axum::{http::Request, middleware::Next, response::Response};
use crate::errors::Result;

pub async fn request_logger<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("Request: {:?}", req.uri());
    Ok(next.run(req).await)
}