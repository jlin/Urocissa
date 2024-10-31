use rocket::{
    form::Form,
    fs::TempFile,
    serde::{json::Json, Serialize},
};
use uuid::Uuid;

use crate::public::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ReturnMessage {
    status: String,
    message: String,
}

#[post("/upload", data = "<data>")]
pub async fn upload(data: Form<Vec<TempFile<'_>>>) -> Json<ReturnMessage> {
    for mut file in data.into_inner() {
        let filename = match file.name() {
            Some(name) => name,
            None => "",
        };
        let extension = match file.content_type() {
            Some(ct) => match ct.extension() {
                Some(ext) => ext.as_str().to_lowercase(),
                None => {
                    return Json(ReturnMessage {
                        status: "error".to_string(),
                        message: "Failed to extract file extension.".to_string(),
                    });
                }
            },
            None => {
                return Json(ReturnMessage {
                    status: "error".to_string(),
                    message: "Failed to get content type.".to_string(),
                });
            }
        };
        if VALID_IMAGE_EXTENSIONS.contains(&extension.as_str())
            || VALID_VIDEO_EXTENSIONS.contains(&extension.as_str())
        {
            let unique_id = Uuid::new_v4();
            let path = format!("./upload/{}-{}.{}", filename, unique_id, extension);
            match file.move_copy_to(path).await {
                Ok(_) => {}
                Err(err) => {
                    return Json(ReturnMessage {
                        status: "error".to_string(),
                        message: format!("Failed to save file: {}", err),
                    });
                }
            }
        } else {
            return Json(ReturnMessage {
                status: "error".to_string(),
                message: "Invalid file type".to_string(),
            });
        }
    }
    return Json(ReturnMessage {
        status: "ok".to_string(),
        message: "Files uploaded successfully!".to_string(),
    });
}
