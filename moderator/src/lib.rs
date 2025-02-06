use bindings::exports::component::content_moderator::moderator::Guest;
use spin_sdk::http::{Request, Response};
use serde_json::json;
use anyhow::Context as _;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn moderate(url: String) -> Result<String, String> {
        println!("checking moderation for {}", url);

        spin_executor::run(moderate(url))
            .map_err(|e| e.to_string())
    }
}

bindings::export!(Component with_types_in bindings);

async fn moderate(image_url: String) -> anyhow::Result<String> {
    let moderate_request_body = json!({
        "model": "omni-moderation-latest",
        "input": [
            {
                "type": "image_url",
                "image_url": {
                    "url": image_url
                }
            }
        ]
    }).to_string();

    let token = spin_sdk::variables::get("token").context("getting token variable")?;

    let moderate_request = Request::post("https://api.openai.com/v1/moderations", moderate_request_body)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .build();

    let response: Response = spin_sdk::http::send(moderate_request).await.context("making moderation request")?;

    Ok(String::from_utf8(response.body().to_vec())?)
}