use init_workshop::{game::Player, server::routes};
use poem::{http::StatusCode, test::TestClient};
use serde_json::json;

#[tokio::test]
async fn create_new_player() {
    let routes = routes();
    let cli = TestClient::new(routes);

    let payload = json!({
        "pseudo": "TestPseudo"
    });

    let response = cli.post("/player").body_json(&payload).send().await;

    response.assert_status(StatusCode::CREATED);

    let json = response.json().await;
    let new_player: Player = json.value().deserialize();

    assert_eq!(new_player.pseudo, "TestPseudo");
    assert_eq!(new_player.level, 0);
}
