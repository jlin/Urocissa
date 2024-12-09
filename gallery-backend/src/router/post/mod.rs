use rocket::Route;

pub mod authenticate;
pub mod create_album;
pub mod post_upload;

pub fn generate_post_routes() -> Vec<Route> {
    routes![
        authenticate::authenticate,
        authenticate::authenticate_share,
        create_album::create_album,
        post_upload::upload
    ]
}
