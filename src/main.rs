use crate::awb::AutonomousWebBrowser;

#[macro_use]
extern crate rocket;

mod awb;
mod requests;

#[launch]
fn rocket() -> _ {
    let _ = AutonomousWebBrowser::browser();
    rocket::build().mount(
        "/",
        routes![
            requests::general::health_check,
            requests::general::page_title,
            requests::general::page_content,
            requests::general::page_screenshot,
            requests::general::page_goto,
            requests::general::page_url,
            requests::input::click,
            requests::input::input_keys,
            requests::input::click_text,
            requests::input::fill,
            requests::input::hover,
            requests::input::input_key,
            requests::find::find,
            requests::find::find_all,
            requests::find::find_text,
            requests::find::find_all_text,
            requests::find::element_text,
            requests::find::element_exists,
            requests::wait::wait,
            requests::wait::wait_for_network_idle
        ],
    )
}
