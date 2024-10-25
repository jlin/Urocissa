use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::uri::Origin;
use rocket::{fairing::AdHoc, http::Method};

use crate::public::config::PUBLIC_CONFIG;

use super::post::authenticate::{Claims, JSON_WEB_TOKEN_SECRET_KEY};

pub fn cache_control_fairing() -> AdHoc {
    AdHoc::on_response("Add Cache-Control header", |req, res| {
        Box::pin(async move {
            // Check if the response status is successful (2xx status codes)
            if res.status().code >= 200 && res.status().code < 300 {
                // Apply cache control headers based on the request path
                if req.uri().path().starts_with("/object")
                    || req.uri().path().starts_with("/assets")
                    || req.uri().path().starts_with("/favicon.ico")
                {
                    res.set_raw_header("Cache-Control", "max-age=31536000, public");
                }
            }
        })
    })
}

pub fn auth_request_fairing() -> AdHoc {
    AdHoc::on_request("Auth Request", |req, _| {
        Box::pin(async move {
            let uri = req.uri().path().to_string();
            if matches!(
                uri.ends_with(".js")
                    || uri.ends_with(".css")
                    || uri.contains("/share")
                    || uri.contains("/assets")
                    || uri.contains("/compressed")
                    || uri.contains("/thumb")
                    || uri == "/login"
                    || uri == "/unauthorized"
                    || uri == "/post/authenticate",
                true
            ) {
                return;
            }
            if PUBLIC_CONFIG.read_only_mode
                && (req.method() != rocket::http::Method::Get && !uri.starts_with("/get/"))
            {
                let forbidden_uri = Origin::parse("/forbidden").unwrap();
                req.set_uri(forbidden_uri);
                return;
            }

            let cookies = req.cookies();
            let jwt_cookie = cookies.get("jwt");

            let auth_pass = {
                if jwt_cookie.is_none() {
                    false
                } else {
                    let token = jwt_cookie.unwrap().value();
                    let validation = Validation::new(Algorithm::HS256);
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(&*JSON_WEB_TOKEN_SECRET_KEY),
                        &validation,
                    ) {
                        Ok(_) => true,
                        Err(_) => {
                            println!("JWT validation failed.");
                            false
                        }
                    }
                }
            };
            if !auth_pass {
                let forbidden_uri = Origin::parse("/unauth").unwrap();
                req.set_method(Method::Get);
                req.set_uri(forbidden_uri);
            }
        })
    })
}
