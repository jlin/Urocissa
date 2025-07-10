use arrayvec::ArrayString;
use jsonwebtoken::{DecodingKey, decode};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::constant::redb::ALBUM_TABLE;
use crate::db::tree::TREE;
use crate::router::claims::claims::Claims;
use crate::router::post::authenticate::JSON_WEB_TOKEN_SECRET_KEY;
use crate::structure::album::ResolvedShare;

use super::VALIDATION;

pub struct GuardShare {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardShare {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let album_id = req.headers().get_one("x-album-id");
        let share_id = req.headers().get_one("x-share-id");

        if let (Some(album_id), Some(share_id)) = (album_id, share_id) {
            let read_txn = TREE.in_disk.begin_read().unwrap();
            let table = read_txn.open_table(ALBUM_TABLE).unwrap();

            if let Some(album_guard) = table.get(album_id).unwrap() {
                let mut album = album_guard.value();

                if let Some(share) = album.share_list.remove(share_id) {
                    let resolved_share = ResolvedShare::new(
                        ArrayString::<64>::from(album_id).unwrap(),
                        album.title,
                        share,
                    );
                    let claims = Claims::new_share(resolved_share);

                    return Outcome::Success(GuardShare { claims });
                }
            }
        }

        if let Some(jwt_cookie) = req.cookies().get("jwt") {
            let token = jwt_cookie.value();
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            ) {
                Ok(token_data) if token_data.claims.is_admin() => {
                    return Outcome::Success(GuardShare {
                        claims: token_data.claims,
                    });
                }
                _ => {
                    warn!("JWT validation failed");
                }
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}
