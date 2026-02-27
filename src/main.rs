#[macro_use]
extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, Serialize, json::Json};

pub mod awb;
use crate::awb::AutonomousWebBrowser;

#[get("/health-check")]
async fn health_check() -> Status {
    match AutonomousWebBrowser::get_page_metadata().await {
        Ok(_) => Status::Ok,
        Err(_) => Status::ServiceUnavailable,
    }
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
    match AutonomousWebBrowser::goto(&request.url).await {
        Ok(_) => Json(ApiResponse {
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

// GET CONTENT

#[get("/get/content")]
async fn get_content() -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::get_page_metadata().await {
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
    match AutonomousWebBrowser::click(&request.selector).await {
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
    match AutonomousWebBrowser::send_keys(&request.selector, &request.keys).await {
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
    AutonomousWebBrowser::screenshot().await
        .map(|bytes| (ContentType::PNG, bytes))
        .map_err(|_| Status::InternalServerError)
}

// GET ELEMENT CONTENT

#[get("/get/content/<selector>")]
async fn get_element_content(selector: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::get_element_content(selector).await {
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
    match AutonomousWebBrowser::wait_for(selector).await {
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
            health_check,
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
