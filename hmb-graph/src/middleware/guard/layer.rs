use axum::{response::Response, body::Body, http::Request};
use tower::{Service, Layer};
use std::task::{Context, Poll};
use async_graphql::futures_util::future::BoxFuture;
use axum::body::HttpBody;

#[derive(Clone)]
struct GuardLayer;

impl<S> Layer<S> for GuardLayer {
    type Service = GuardMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GuardMiddleware { inner }
    }
}

struct GuardMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for GuardMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        // let mut req = request;
        let future = self.inner.call(request);
        Box::pin(async move {

            let response: Response = future.await?;
            Ok(response)
        })
    }
}