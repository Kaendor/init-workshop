use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use init_workshop::{game::Player, server::routes};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn create_new_player() {
    let routes = routes();

    let payload = json!({
        "pseudo": "TestPseudo"
    });

    let response = routes
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/player")
                .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&payload).expect("value to bytes"),
                ))
                .unwrap(),
        )
        .await
        .expect("request to be done");

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body in response")
        .to_bytes();
    let new_player: Player = serde_json::from_slice(&body).expect("deserialization of body");

    assert_eq!(new_player.pseudo, "TestPseudo");
    assert_eq!(new_player.level, 0);
}
