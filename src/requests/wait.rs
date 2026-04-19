use rocket::serde::json::Json;

use crate::awb::AutonomousWebBrowser;
use crate::requests::ApiResponse;

#[get("/wait/<delay>", format = "application/json")]
pub async fn wait(delay: u64) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::wait(delay).await {
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

#[get("/wait/network/idle/<delay>/<timeout>", format = "application/json")]
pub async fn wait_for_network_idle(delay: u64, timeout: u64) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::wait_for_network_idle(delay, timeout).await {
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
