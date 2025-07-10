pub mod claims;
pub mod delete;
pub mod fairing;
pub mod get;
pub mod post;
pub mod put;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_json::json;
use std::io::Cursor;

pub struct AppError(anyhow::Error);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
        // 1. 最外層訊息（即 context() 加的那層）
        let outer_msg = self.0.to_string();

        // 2. 收集整條 error-chain
        let chain: Vec<String> = self
            .0
            .chain() // ← 直接取得 Chain
            .map(|e| e.to_string())
            .collect();

        // 3. 組 JSON
        let body = json!({
            "error": outer_msg,
            "chain": chain,         // 也可改成 .join(": ") 變單一字串
        })
        .to_string();

        Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

// 仍然保留自動 From<anyhow::Error>
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
