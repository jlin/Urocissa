use rocket::Route;

pub mod edit_album;
pub mod edit_tag;
pub mod random;
pub mod regenerate_metadata;
pub mod regenerate_thumbnail;
pub fn generate_put_routes() -> Vec<Route> {
    routes![
        edit_album::edit_album,
        edit_album::set_album_cover,
        edit_album::set_album_title,
        edit_tag::edit_tag,
        random::generate_random_data,
        regenerate_thumbnail::regenerate_thumbnail,
        regenerate_thumbnail::regenerate_thumbnail_with_frame,
        regenerate_metadata::regenerate_metadata,
    ]
}
