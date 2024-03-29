use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use init_workshop::server::routes;
use tower::ServiceExt;

#[tokio::test]
fn create_new_player() {
    let routes = routes();

    let response = routes
        .oneshot(
            Request::builder()
                .uri("/player")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
}
