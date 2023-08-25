//! An example showing how to send a single event to June.

use june_analytics::message::{Track, User};
use june_analytics::{Client, HttpClient};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    client
        .send(
            write_key.to_string(),
            Track {
                user: User::UserId {
                    user_id: "some_user_id".to_owned(),
                },
                event: "Example Event".to_owned(),
                properties: json!({
                    "some property": "some value",
                    "some other property": "some other value",
                }),
                ..Default::default()
            }
            .into(),
        )
        .await
        .expect("could not send to June");
}
