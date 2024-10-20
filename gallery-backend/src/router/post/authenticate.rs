use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::post;
use rocket::serde::json::Json;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::public::config::PRIVATE_CONFIG;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
}

#[post("/post/authenticate", data = "<password>")]
pub async fn authenticate(password: Json<String>) -> Result<Json<String>, &'static str> {
    let input_password = password.into_inner();

    // Verify the password (hardcoded for a single user)
    if input_password == PRIVATE_CONFIG.password {
        // Create expiration timestamp (valid for 1 hour)
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 3600;

        // Generate claims
        let claims = Claims {
            exp: expiration as usize,
        };

        // Encode the JWT token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(PRIVATE_CONFIG.password.as_ref()),
        )
        .map_err(|_| "Token generation failed")?;

        return Ok(Json(token)); // Return the JWT token
    }

    Err("Invalid password")
}
