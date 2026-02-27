use rocket::serde::Serialize;

pub mod find;
pub mod general;
pub mod input;
pub mod wait;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<Type> {
    success: bool,
    data: Option<Type>,
    error: Option<String>,
}
