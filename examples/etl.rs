//! An example showing how to do an ETL-like operation loading events into
//! June.

use june_analytics::batcher::Batcher;
use june_analytics::client::Client;
use june_analytics::http::HttpClient;
use june_analytics::message::{BatchMessage, Track, User};
use serde_json::json;

fn main() {
    let write_key = "YOUR_WRITE_KEY";

    let client = HttpClient::default();
    let mut batcher = Batcher::new(None);

    // Pretend this is reading off of a queue, a file, or some other data
    // source.
    for i in 0..100 {
        let msg = BatchMessage::Track(Track {
            user: User::UserId {
                user_id: format!("user-{}", i),
            },
            event: "Example Event".to_owned(),
            properties: json!({
                "foo": format!("bar-{}", i),
            }),
            ..Default::default()
        });

        // An error here indicates a message is too large. In real life, you
        // would probably want to put this message in a deadletter queue or some
        // equivalent.
        if let Some(msg) = batcher.push(msg).unwrap() {
            client.send(write_key, &batcher.into_message()).unwrap();

            batcher = Batcher::new(None);
            batcher.push(msg).unwrap(); // Same error condition as above.
        }
    }
}
