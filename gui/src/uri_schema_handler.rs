use serde_json::json;
use tauri::{
    http::{Request, Response, ResponseBuilder},
    AppHandle, Manager,
};

pub fn uri_schema_handler<'a, 'b>(
    handle: &'a AppHandle,
    request: &'b Request,
) -> Result<Response, Box<dyn std::error::Error + 'static>> {
    let uri = request.uri().to_string();

    match uri.as_str() {
        "askai://show" => {
            let win = handle.get_window("main").unwrap();
            win.show().unwrap();
            win.set_focus().unwrap();
        }
        "askai://quit" => handle.exit(0),
        _ => (),
    }

    let response = ResponseBuilder::new()
        .status(200)
        .mimetype("application/json")
        .body(json!({}).to_string().into_bytes())
        .unwrap();

    Ok(response)
}
