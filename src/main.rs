#[macro_use]
extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, Serialize, json::Json};

pub mod browser_singleton;
use crate::browser_singleton::BrowserSingleton;

#[get("/health-check")]
async fn index() -> String {
    String::from("okay")
}

#[get("/repeat/<message>/<times>")]
async fn repeat(message: &str, times: u8) -> String {
    let mut output = String::new();
    for _ in 0..times {
        output.push_str(message);
        output.push('\n');
    }
    output
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse<Type> {
    success: bool,
    data: Option<Type>,
    error: Option<String>,
}

// GOTO REQUEST

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GotoRequest {
    url: String,
}

#[post("/goto", format = "application/json", data = "<request>")]
async fn goto(request: Json<GotoRequest>) -> Json<ApiResponse<String>> {
    match BrowserSingleton::goto(&request.url) {
        Ok(_) => match BrowserSingleton::get_content() {
            Ok(html) => Json(ApiResponse {
                success: true,
                data: Some(html),
                error: None,
            }),
            Err(error) => Json(ApiResponse {
                success: false,
                data: None,
                error: Some(error.to_string()),
            }),
        },
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

// GET CONTENT

#[get("/get/content")]
async fn get_content() -> Json<ApiResponse<String>> {
    match BrowserSingleton::get_content() {
        Ok(html) => Json(ApiResponse {
            success: true,
            data: Some(html),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

// CLICK ROUTE

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ClickRequest {
    selector: String,
}

#[post("/click", data = "<request>", format = "application/json")]
async fn click(request: Json<ClickRequest>) -> Json<ApiResponse<String>> {
    match BrowserSingleton::click(&request.selector) {
        Ok(()) => Json(ApiResponse {
            success: true,
            data: None,
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

// KEYS ROUTE

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct KeysRequest {
    selector: String,
    keys: String,
}

#[post("/send/keys", data = "<request>", format = "application/json")]
async fn send_keys(request: Json<KeysRequest>) -> Json<ApiResponse<String>> {
    match BrowserSingleton::send_keys(&request.selector, &request.keys) {
        Ok(()) => Json(ApiResponse {
            success: true,
            data: None,
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

// SCREENSHOT ROUTE

#[get("/screenshot")]
async fn screenshot() -> Result<(ContentType, Vec<u8>), Status> {
    BrowserSingleton::screenshot()
        .map(|bytes| (ContentType::PNG, bytes))
        .map_err(|_| Status::InternalServerError)
}

// GET ELEMENT CONTENT

#[get("/get/content/<selector>")]
async fn get_element_content(selector: &str) -> Json<ApiResponse<String>> {
    match BrowserSingleton::get_element_content(selector) {
        Ok(content) => Json(ApiResponse {
            success: true,
            data: Some(content),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

// Wait For Element

#[get("/wait/for/<selector>")]
async fn wait_for_element(selector: &str) -> Json<ApiResponse<String>> {
    match BrowserSingleton::wait_for(selector) {
        Ok(element) => Json(ApiResponse {
            success: true,
            data: Some(String::from("Present")),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            repeat,
            goto,
            click,
            send_keys,
            get_content,
            wait_for_element,
            get_element_content,
            screenshot
        ],
    )
}
