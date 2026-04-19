use rocket::http::{ContentType, Status};
use rocket::serde::{Deserialize, json::Json};

use crate::awb::AutonomousWebBrowser;
use crate::requests::ApiResponse;

#[get("/health-check", format = "application/json")]
pub async fn health_check() -> Status {
    match AutonomousWebBrowser::page_title().await {
        Ok(_) => Status::Ok,
        Err(_) => Status::ServiceUnavailable,
    }
}

#[get("/page/title", format = "application/json")]
pub async fn page_title() -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::page_title().await {
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

#[get("/page/url", format = "application/json")]
pub async fn page_url() -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::page_url().await {
        Ok(url) => Json(ApiResponse {
            success: true,
            data: Some(url),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

#[get("/page/content", format = "application/json")]
pub async fn page_content() -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::page_content().await {
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

#[get("/page/screenshot")]
pub async fn page_screenshot() -> Result<(ContentType, Vec<u8>), Status> {
    AutonomousWebBrowser::page_screenshot()
        .await
        .map(|bytes| (ContentType::PNG, bytes))
        .map_err(|_| Status::InternalServerError)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GotoRequest {
    url: String,
}

#[post("/page/goto", data = "<request>")]
pub async fn page_goto(request: Json<GotoRequest>) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::page_goto(&request.url).await {
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
