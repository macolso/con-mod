use bindings::component::content_moderator::moderator;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
fn handle(req: Request) -> anyhow::Result<impl IntoResponse> {
    let body = String::from_utf8(req.body().to_vec()).unwrap();

    match moderator::moderate(&body) {
        Ok(result) => {
            Ok(Response::new(200, result))
        }
        Err(err) => {
            Ok(Response::new(500, err.to_string()))
        }
    }
}

mod bindings {
    wit_bindgen::generate!({
        inline: r"
            package component:content-moderator;

            interface moderator {
                moderate: func(url: string) -> result<string, string>;
            }

            world content-moderator {
                import moderator;
            }
        ",
    });
}