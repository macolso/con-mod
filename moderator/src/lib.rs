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

async fn moderate(url: String) -> anyhow::Result<String> {
    let document = fetch_content(url).await?;

    let moderate_request_body = json!({"input": document}).to_string();

    let token = spin_sdk::variables::get("token").context("getting token variable")?;

    let moderate_request = Request::post("https://api.openai.com/v1/moderations", moderate_request_body)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .build();

    let response: Response = spin_sdk::http::send(moderate_request).await.context("making moderation request")?;

    Ok(String::from_utf8(response.body().to_vec())?)
}

async fn fetch_content(url: String) -> anyhow::Result<String> {
    let content: Response = spin_sdk::http::send(Request::get(url)).await.context("fetching content")?;

    println!("Content fetch status: {}", content.status());

    if *content.status() != 200 {
        return Err(anyhow::anyhow!("failed to fetch content, got {}", content.status()));
    }

    // convert the html response to text format
    let document = html2text::from_read(&mut content.body(), 20).context("converting html to text")?;

    Ok(document)
}