use rocket::serde::json::Json;

use crate::requests::ApiResponse;
use crate::awb::AutonomousWebBrowser;

#[get("/find/<selector>")]
pub async fn find(selector: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::find(selector).await {
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

#[get("/find/all/<selector>")]
pub async fn find_all(selector: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::find_all(selector).await {
        Ok(content) => Json(ApiResponse {
            success: true,
            data: Some( String::from( "WIP" )),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

#[get("/find/text/<text>")]
pub async fn find_text(text: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::find_text(text).await {
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


#[get("/find/all/text/<text>")]
pub async fn find_all_text(text: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::find_all_text(text).await {
        Ok(content) => Json(ApiResponse {
            success: true,
            data: Some( String::from("WIP") ),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

#[get("/find/element-text/<selector>")]
pub async fn element_text(selector: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::element_text(selector).await {
        Ok(text) => Json(ApiResponse {
            success: true,
            data: Some(text),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}

#[get("/find/exists/<selector>")]
pub async fn element_exists(selector: &str) -> Json<ApiResponse<String>> {
    match AutonomousWebBrowser::element_exists(selector).await {
        Ok(exists) => Json(ApiResponse {
            success: true,
            data: Some(exists.to_string()),
            error: None,
        }),
        Err(error) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(error.to_string()),
        }),
    }
}
