use std::time::{SystemTime, UNIX_EPOCH};

use arrayvec::ArrayString;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use rocket::Request;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::public::redb::ALBUM_TABLE;
use crate::public::tree::TREE;
use crate::router::post::authenticate::JSON_WEB_TOKEN_SECRET_KEY;

use super::VALIDATION;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub album_id: Option<ArrayString<64>>,
    pub exp: u64,
}

impl Claims {
    pub fn new() -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 86400 * 14; // 14 days

        Self {
            album_id: None,
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

pub struct GuardAuth {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardAuth {
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
                Ok(token_data_claims) => {
                    let claims = token_data_claims.claims;
                    return Outcome::Success(GuardAuth { claims });
                }
                _ => {
                    warn!("JWT validation failed.");
                }
            }
        // Check for share mode
        } else if let (Some(album_cookie), Some(share_cookie)) =
            (cookies.get("albumId"), cookies.get("shareId"))
        {
            let album_id = album_cookie.value();
            let share_id = share_cookie.value();
            info!(
                "Extracted album_id: {} and share_id: {}",
                album_id, share_id
            );
            let read_txn = TREE.in_disk.begin_read().unwrap();
            let table = read_txn.open_table(ALBUM_TABLE).unwrap();
            /* if let Some(album) = table.get(&*album_id).unwrap() {
                let share = album.value().share_list
                let mut claims = Claims::new();
                claims.album_id = ArrayString::from(album_id).ok();
                return Outcome::Success(GuardAuth { claims });
            } */
            let mut claims = Claims::new();
            claims.album_id = ArrayString::from(album_id).ok();
            return Outcome::Success(GuardAuth { claims });
        }

        return Outcome::Forward(Status::Unauthorized);
    }
}
