extern crate june_analytics;
use dotenv::dotenv;
use june_analytics::message::{Message, Track, User};
use june_analytics::Client;
use serde_json::json;

#[tokio::test]
async fn run_analytics() {
    dotenv().ok();
    let key = std::env::var("JUNE_WRITE_KEY").unwrap();
    let event_name = "Create Embeding";
    let id = "64b1b773cb5c1e6ea6b59c1e";
    let org = "64b1b773cb5c1e6ea6b59c1d";

    let june_client = june_analytics::HttpClient::default();

    let result = june_client
        .send(
            key.to_string(),
            Message::Track(Track {
                user: User::UserId {
                    user_id: id.to_owned(),
                },
                event: event_name.to_string(),
                context: Some(json!({
                    "groupId": org.to_string(),
                })),
                ..Default::default()
            }),
        )
        .await;

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        // Fail the test and print error
        assert!(false == true, "Error: {:?}", e);
    } else {
        eprintln!("{:?}", result);
        assert!(result.is_ok());
    }
}
