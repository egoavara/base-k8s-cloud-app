use std::task::{Context, Poll};
use async_graphql::futures_util::future::BoxFuture;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use tower::{Layer, Service};
use crate::middleware::guard::Hint;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum RbacGuard{
    Equal(String),
    OneOf(Vec<String>),
}

impl RbacGuard{
    pub fn equal<S : Into<String>>(s : S)-> Self{
        RbacGuard::Equal(s.into())
    }
    pub fn oneof<S: Into<String>, I : IntoIterator<Item=S>>(iter : I)-> Self{
        RbacGuard::OneOf(iter.into_iter()
                             .map(|x|x.into())
                             .collect())
    }
}

impl <S>Layer<S> for RbacGuard{
    type Service = RbacGuardMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RbacGuardMiddleware{
            inner,
            roles: match self {
                RbacGuard::Equal(eq) => vec![eq.to_string()],
                RbacGuard::OneOf(one) => one.clone(),
            }
        }
    }
}

#[derive(Clone)]
pub struct RbacGuardMiddleware<S> {
    inner: S,
    roles: Vec<String>
}

impl<S> Service<Request<Body>> for RbacGuardMiddleware<S>
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
                if self.roles
                       .iter()
                       .any(|x| hint.role.contains(x)){
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