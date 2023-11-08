use std::task::{Context, Poll};
use async_graphql::futures_util::future::BoxFuture;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use tower::{Layer, Service};
use crate::middleware::guard::Hint;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum AbacGuard {
    Equal(String),
    OneOf(Vec<String>),
}

impl AbacGuard {
    pub fn equal<S : Into<String>>(s : S)-> Self{
        AbacGuard::Equal(s.into())
    }
    pub fn oneof<S: Into<String>, I : IntoIterator<Item=S>>(iter : I)-> Self{
        AbacGuard::OneOf(iter.into_iter()
                             .map(|x|x.into())
                             .collect())
    }
}

impl <S>Layer<S> for AbacGuard {
    type Service = AbacGuardMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AbacGuardMiddleware {
            inner,
            attributes: match self {
                AbacGuard::Equal(eq) => vec![eq.to_string()],
                AbacGuard::OneOf(one) => one.clone(),
            }
        }
    }
}

pub  struct AbacGuardMiddleware<S> {
    inner: S,
    attributes: Vec<String>
}

impl<S> Service<Request<Body>> for AbacGuardMiddleware<S>
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

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        match request.extensions().get::<Hint>(){
            None => {
                return Box::pin(async move {
                    Ok(StatusCode::UNAUTHORIZED.into_response())
                })
            }
            Some(hint) => {
                if self.attributes
                       .iter()
                       .any(|x| hint.attribute.contains(x)){
                    let future = self.inner.call(request);
                    Box::pin(async move {
                        Ok(future.await?)
                    })
                }else{
                    return Box::pin(async move {
                        Ok(StatusCode::FORBIDDEN.into_response())
                    })
                }
            }
        }
    }
}