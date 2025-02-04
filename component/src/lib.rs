use bindings::exports::component::content_moderator::moderator::Guest;
use spin_sdk::http::{Request, Response};

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn moderate(url: String) -> Result<String, String> {
        todo!("implement me")
    }
}

bindings::export!(Component with_types_in bindings);