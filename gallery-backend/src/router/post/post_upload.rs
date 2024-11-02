use crate::public::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS};
use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::{
    form::Form,
    fs::TempFile,
    serde::{json::Json, Serialize},
};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ReturnMessage {
    status: String,
    message: String,
}
pub enum FileUpload<'r> {
    LastModified(u64),
    File(TempFile<'r>),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FileUpload<'r> {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        // 使用 u64 已實現的 from_value 方法
        match u64::from_value(field) {
            Ok(value) => Ok(FileUpload::LastModified(value)),
            Err(err) => Err(err),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // 使用 TempFile 已實現的 from_data 方法
        match TempFile::from_data(field).await {
            Ok(temp_file) => Ok(FileUpload::File(temp_file)),
            Err(err) => Err(err),
        }
    }
}

#[post("/upload", data = "<data>")]
pub async fn upload(data: Form<Vec<FileUpload<'_>>>) -> Json<ReturnMessage> {
    let mut last_modified_time = 0;
    for file_data in data.into_inner() {
        match file_data {
            FileUpload::LastModified(last_modified_time_received) => {
                last_modified_time = last_modified_time_received;
            }
            FileUpload::File(mut file) => {
                let filename = match file.name() {
                    Some(name) => name.to_string(),
                    None => "".to_string(),
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
                    let path_tmp = format!("./upload/{}-{}.tmp", filename, unique_id);
                    match file.move_copy_to(&path_tmp).await {
                        Ok(_) => {
                            if let Err(err) = filetime::set_file_mtime(
                                &path_tmp,
                                filetime::FileTime::from_unix_time(
                                    (last_modified_time / 1000) as i64,
                                    0,
                                ),
                            ) {
                                return Json(ReturnMessage {
                                    status: "error".to_string(),
                                    message: format!("Failed to set last modified time: {}", err),
                                });
                            }
                            // 重新命名檔案
                            let path_final =
                                format!("./upload/{}-{}.{}", filename, unique_id, extension);
                            if let Err(err) = std::fs::rename(&path_tmp, &path_final) {
                                return Json(ReturnMessage {
                                    status: "error".to_string(),
                                    message: format!("Failed to rename file: {}", err),
                                });
                            }
                        }
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
        }
    }
    return Json(ReturnMessage {
        status: "ok".to_string(),
        message: "Files uploaded successfully!".to_string(),
    });
}
