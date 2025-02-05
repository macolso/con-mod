use bindings::exports::component::content_moderator::moderator::Guest;
use html2text::from_read;
use spin_sdk::http::{Request, Response};

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn moderate(url: String) -> Result<String, String> {
        // hardcode url for testing
        let url = "https://www.engadget.com/2017-05-15-not-hotdog-app-hbo-silicon-valley.html";

        // fetch the url
        let result: Result<Response, _> =
            spin_executor::run(spin_sdk::http::send(Request::get(url)));

        let resp = result.unwrap();

        // convert the html response to text format
        let document = from_read(&resp.body()[..], 80).unwrap();

        // dump for testing
        println!("{document}");

        Ok(String::from("done"))
    }
}

bindings::export!(Component with_types_in bindings);
