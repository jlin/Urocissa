use arrayvec::ArrayString;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use rocket::Request;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::public::album::ResolvedShare;
use crate::public::redb::ALBUM_TABLE;
use crate::public::tree::TREE;
use crate::router::post::authenticate::JSON_WEB_TOKEN_SECRET_KEY;

use super::VALIDATION;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub resolved_share_opt: Option<ResolvedShare>,
    pub exp: u64,
}

impl Claims {
    pub fn new() -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 14 * 86_400; // 14 days

        Self {
            resolved_share_opt: None,
            exp,
        }
    }

    pub fn encode(&self) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
        )
        .expect("Failed to generate token")
    }
}

pub struct GuardAuthShare {
    pub claims: Claims,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardAuthShare {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // 優先處理 JWT cookie
        if let Some(jwt_cookie) = req.cookies().get("jwt") {
            let token = jwt_cookie.value();
            if let Ok(token_data_claims) = decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            ) {
                return Outcome::Success(GuardAuthShare {
                    claims: token_data_claims.claims,
                });
            }
            warn!("JWT validation failed.");
        }

        // 改從 headers 讀取 albumId / shareId
        let album_id = req.headers().get_one("x-album-id");
        let share_id = req.headers().get_one("x-share-id");

        if let (Some(album_id), Some(share_id)) = (album_id, share_id) {
            info!("Received album_id={} and share_id={}", album_id, share_id);

            let read_txn = TREE.in_disk.begin_read().unwrap();
            let table = read_txn.open_table(ALBUM_TABLE).unwrap();

            if let Some(album_guard) = table.get(album_id).unwrap() {
                let mut album = album_guard.value();

                if let Some(share) = album.share_list.remove(share_id) {
                    let mut claims = Claims::new();
                    let resolved_share = ResolvedShare::new(
                        ArrayString::<64>::from(album_id).unwrap(),
                        album.title,
                        share,
                    );
                    claims.resolved_share_opt = Some(resolved_share);

                    return Outcome::Success(GuardAuthShare { claims });
                }
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}

pub struct GuardAuthUpload;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardAuthUpload {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(jwt_cookie) = req.cookies().get("jwt") {
            let token = jwt_cookie.value();
            if decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            )
            .is_ok()
            {
                return Outcome::Success(GuardAuthUpload);
            }
            warn!("JWT validation failed.");
        }

        let album_id = req.headers().get_one("x-album-id");
        let share_id = req.headers().get_one("x-share-id");

        if let (Some(album_id), Some(share_id)) = (album_id, share_id) {
            info!("Received album_id={} and share_id={}", album_id, share_id);

            let read_txn = TREE.in_disk.begin_read().unwrap();
            let table = read_txn.open_table(ALBUM_TABLE).unwrap();

            if let Some(album_guard) = table.get(album_id).unwrap() {
                let album = album_guard.value();

                if let Some(share) = album.share_list.get(share_id) {
                    if share.show_upload {
                        return Outcome::Success(GuardAuthUpload);
                    } else {
                        warn!("Share exists, but upload not allowed.");
                    }
                }
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}

pub struct GuardAuthEdit;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardAuthEdit {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Check for JWT cookie
        let cookies: &CookieJar = req.cookies();
        if let Some(jwt_cookie) = cookies.get("jwt") {
            let token = jwt_cookie.value();

            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            ) {
                Ok(_) => {
                    return Outcome::Success(GuardAuthEdit);
                }
                _ => {
                    warn!("JWT validation failed.");
                }
            }
            // No need to check for share mode
        }
        return Outcome::Forward(Status::Unauthorized);
    }
}
