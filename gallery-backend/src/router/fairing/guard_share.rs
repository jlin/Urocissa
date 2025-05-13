use arrayvec::ArrayString;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::public::album::ResolvedShare;
use crate::public::redb::ALBUM_TABLE;
use crate::looper::tree::TREE;
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
                    let mut claims = Claims::new();
                    let resolved_share = ResolvedShare::new(
                        ArrayString::<64>::from(album_id).unwrap(),
                        album.title,
                        share,
                    );
                    claims.resolved_share_opt = Some(resolved_share);

                    return Outcome::Success(GuardShare { claims });
                }
            }
        }

        if let Some(jwt_cookie) = req.cookies().get("jwt") {
            let token = jwt_cookie.value();
            if let Ok(token_data_claims) = decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            ) {
                return Outcome::Success(GuardShare {
                    claims: token_data_claims.claims,
                });
            }
            warn!("JWT validation failed.");
        }

        Outcome::Forward(Status::Unauthorized)
    }
}
