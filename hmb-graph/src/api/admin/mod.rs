use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::middleware::guard::RbacGuard;

async fn test()-> impl IntoResponse{
    return "hello, world"
}
pub fn router()->Router{
    let a = get(test);
    Router::new()
        .route("/a", a)
        .layer(RbacGuard::equal("aaa"))
}