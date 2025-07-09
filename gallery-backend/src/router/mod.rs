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

// 1. 建立一個專門的錯誤型別來包裝 anyhow::Error
pub struct AppError(anyhow::Error);

// 2. 為我們的錯誤型別實作 Responder
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'static> {
        // 將 anyhow::Error 轉換為 JSON 回應
        let error_message = self.0.to_string();
        let error_response = json!({ "error": error_message }).to_string();

        Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::JSON)
            .sized_body(error_response.len(), Cursor::new(error_response))
            .ok()
    }
}

// 3. 實作 From<anyhow::Error>，這是讓 `?` 運算子工作的關鍵
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(err)
    }
}
