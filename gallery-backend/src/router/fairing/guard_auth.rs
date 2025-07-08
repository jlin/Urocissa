use crate::router::claims::claims::Claims;
use crate::router::post::authenticate::JSON_WEB_TOKEN_SECRET_KEY;
use jsonwebtoken::{DecodingKey, decode};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use super::VALIDATION;

pub struct GuardAuth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(jwt_cookie) = req.cookies().get("jwt") {
            let token = jwt_cookie.value();
            match decode::<Claims>(
                token,
                &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                &VALIDATION,
            ) {
                Ok(claims_data) if claims_data.claims.is_admin() => {
                    return Outcome::Success(GuardAuth);
                }
                _ => {
                    warn!("JWT validation failed.");
                }
            }
        }
        return Outcome::Forward(Status::Unauthorized);
    }
}
