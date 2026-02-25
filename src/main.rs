#[macro_use]
extern crate rocket;
use rocket::http::ContentType;
use rocket::serde::{Deserialize, json::Json};

pub mod browser_singleton;
use crate::browser_singleton::BrowserSingleton;

const OKAY: &str = "Okay";

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

// GOTO REQUEST

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GotoRequest {
    url: String,
}

#[post("/goto", format = "application/json", data = "<request>")]
async fn goto(request: Json<GotoRequest>) -> String {
    match BrowserSingleton::goto(&request.url) {
        Ok(()) => BrowserSingleton::get_content().unwrap(),
        Err(error) => error.to_string(),
    }
}

// GET CONTENT

#[get("/get/content")]
async fn get_content() -> String {
    BrowserSingleton::get_content().expect("Could Not Get Content")
}

// CLICK ROUTE

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ClickRequest {
    selector: String,
}

#[post("/click", data = "<request>", format = "application/json")]
async fn click(request: Json<ClickRequest>) -> String {
    match BrowserSingleton::click(&request.selector) {
        Ok(()) => String::from(OKAY),
        Err(error) => error.to_string(),
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
async fn send_keys(request: Json<KeysRequest>) -> String {
    match BrowserSingleton::send_keys(&request.selector, &request.keys) {
        Ok(()) => String::from(OKAY),
        Err(error) => error.to_string(),
    }
}

// SCREENSHOT ROUTE

#[get("/screenshot")]
async fn screenshot() -> (ContentType, Vec<u8>) {
    (ContentType::PNG, BrowserSingleton::screenshot())
}

// GET ELEMENT CONTENT

#[get("/get/content/<selector>")]
async fn get_element_content(selector: &str) -> String {
    match BrowserSingleton::get_element_content(selector) {
        Ok(result) => result,
        Err(error) => error.to_string(),
    }
}

// Wait For Element

#[get("/wait/for/<selector>")]
async fn wait_for_element(selector: &str) -> String {
    match BrowserSingleton::wait_for(selector) {
        Ok(result) => result.get_content().unwrap(),
        Err(error) => error.to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    let _ = BrowserSingleton::browser();
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
