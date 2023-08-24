//! An example showing how to send a single event to June.

use june_analytics::client::Client;
use june_analytics::http::HttpClient;
use june_analytics::message::{Message, Track, User};
use serde_json::json;

fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    client
        .send(
            write_key,
            &Message::Track(Track {
                user: User::UserId {
                    user_id: "some_user_id".to_owned(),
                },
                event: "Example Event".to_owned(),
                properties: json!({
                    "some property": "some value",
                    "some other property": "some other value",
                }),
                ..Default::default()
            }),
        )
        .expect("could not send to June");
}
