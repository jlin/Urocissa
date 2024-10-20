use rocket::fairing::AdHoc;
use rocket::http::uri::Origin;

use crate::public::config::PRIVATE_CONFIG;

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
            let uri = req.uri().to_string();
            if matches!(
                uri.ends_with(".js")
                    || uri.ends_with(".css")
                    || uri.contains("/share")
                    || uri.contains("/assets")
                    || uri.contains("/compressed")
                    || uri.contains("/thumb"),
                true
            ) {
                return;
            }
            if PRIVATE_CONFIG.read_only_mode
                && (req.method() != rocket::http::Method::Get && !uri.starts_with("/get/"))
            {
                let forbidden_uri = Origin::parse("/forbidden").unwrap();
                req.set_uri(forbidden_uri);
                return;
            }
            let cookies = req.cookies();
            let password_cookie = cookies.get("password");
            if (req.uri().path() != "/login" && req.uri().path() != "/post/authenticate")
                && (password_cookie.is_none()
                    || password_cookie.unwrap().value() != PRIVATE_CONFIG.password)
            {
                let forbidden_uri = Origin::parse("/redirect-to-login").unwrap();
                req.set_uri(forbidden_uri);
            }
        })
    })
}
