use axum::{
  http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, Request, StatusCode},
  middleware::Next,
  response::Response,
};

pub async fn handler<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  let mut response = next.run(req).await;

  response
    .headers_mut()
    .insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

  Ok(response)
}
