use crate::router::fairing::guard_share::Claims;
use crate::router::post::authenticate::JSON_WEB_TOKEN_SECRET_KEY;
use jsonwebtoken::{DecodingKey, decode};
use rocket::Request;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};

use super::VALIDATION;

pub struct GuardAuth;

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
                Ok(_) => {
                    return Outcome::Success(GuardAuth);
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
