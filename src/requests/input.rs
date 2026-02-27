use rocket::serde::{Deserialize, json::Json};

use crate::requests::ApiResponse;
use crate::awb::AutonomousWebBrowser;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ClickRequest {
    selector: String,
}

#[post("/input/click", data = "<request>", format = "application/json")]
pub async fn click(request: Json<ClickRequest>) -> Json<ApiResponse<String>> {
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


#[post("/input/click/text", data = "<request>", format = "application/json")]
pub async fn click_text(request: Json<ClickRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::click_text(&request.selector).await {
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

#[post("/input/hover", data = "<request>", format = "application/json")]
pub async fn hover(request: Json<ClickRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::hover(&request.selector).await {
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
pub struct KeysRequest {
    selector: String,
    keys: String,
}

#[post("/input/keys", data = "<request>", format = "application/json")]
pub async fn input_keys(request: Json<KeysRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::input_keys(&request.selector, &request.keys).await {
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

#[post("/input/fill", data = "<request>", format = "application/json")]
pub async fn fill(request: Json<KeysRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::fill(&request.selector, &request.keys).await {
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct KeyRequest {
    key: String,
}

#[post("/input/key", data = "<request>", format = "application/json")]
pub async fn input_key(request: Json<KeyRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::input_key(&request.key).await {
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
