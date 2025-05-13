use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use super::abstract_data::AbstractData;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Expression {
    Or(Vec<Expression>),
    And(Vec<Expression>),
    Not(Box<Expression>),
    Tag(String),
    ExtType(String),
    Ext(String),
    Model(String),
    Make(String),
    Path(String),
    Album(ArrayString<64>),
    Any(String),
}

impl Expression {
    pub fn generate_filter(self) -> Box<dyn Fn(&AbstractData) -> bool + Sync + Send> {
        match self {
            Expression::Or(expressions) => {
                let filters: Vec<Expression> = expressions;
                Box::new(move |abstract_data: &AbstractData| {
                    filters.iter().any(|expr| {
                        let filter = expr.clone().generate_filter();
                        filter(abstract_data)
                    })
                })
            }
            Expression::And(expressions) => {
                let filters: Vec<Expression> = expressions;
                Box::new(move |abstract_data: &AbstractData| {
                    filters.iter().all(|expr| {
                        let filter = expr.clone().generate_filter();
                        filter(abstract_data)
                    })
                })
            }
            Expression::Not(expression) => {
                let inner_filter = expression.clone().generate_filter();
                Box::new(move |abstract_data: &AbstractData| !inner_filter(abstract_data))
            }
            Expression::Tag(tag) => {
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => db.tag.contains(&tag),
                    AbstractData::Album(album) => album.tag.contains(&tag),
                })
            }
            Expression::ExtType(ext_type) => {
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => db.ext_type.contains(&ext_type),
                    AbstractData::Album(_) => ext_type.contains("album"),
                })
            }
            Expression::Ext(ext) => {
                let ext_lower = ext.to_ascii_lowercase();
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => db.ext.to_ascii_lowercase().contains(&ext_lower),
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Model(model) => {
                let model_lower = model.to_ascii_lowercase();
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => {
                        db.exif_vec.get("Model").map_or(false, |model_of_exif| {
                            model_of_exif.to_ascii_lowercase().contains(&model_lower)
                        })
                    }
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Make(make) => {
                let make_lower = make.to_ascii_lowercase();
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => {
                        db.exif_vec.get("Make").map_or(false, |make_of_exif| {
                            make_of_exif.to_ascii_lowercase().contains(&make_lower)
                        })
                    }
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Path(path) => {
                let path_lower = path.to_ascii_lowercase();
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => db.alias.iter().any(|file_modify| {
                        file_modify.file.to_ascii_lowercase().contains(&path_lower)
                    }),
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Album(album_id) => {
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => db.album.contains(&album_id),
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Any(any_identifier) => {
                let any_lower = any_identifier.to_ascii_lowercase();
                Box::new(move |abstract_data: &AbstractData| match abstract_data {
                    AbstractData::Database(db) => {
                        db.tag.contains(&any_identifier)
                            || db.ext_type.contains(&any_identifier)
                            || db.ext.to_ascii_lowercase().contains(&any_lower)
                            || db.exif_vec.get("Make").map_or(false, |make_of_exif| {
                                make_of_exif.to_ascii_lowercase().contains(&any_lower)
                            })
                            || db.exif_vec.get("Model").map_or(false, |model_of_exif| {
                                model_of_exif.to_ascii_lowercase().contains(&any_lower)
                            })
                            || db.alias.iter().any(|file_modify| {
                                file_modify.file.to_ascii_lowercase().contains(&any_lower)
                            })
                    }
                    AbstractData::Album(album) => {
                        album.tag.contains(&any_identifier)
                            || "album".to_ascii_lowercase().contains(&any_lower)
                    }
                })
            }
        }
    }
    pub fn generate_filter_hide_metadata(
        self,
        shared_album_id: ArrayString<64>,
    ) -> Box<dyn Fn(&AbstractData) -> bool + Send + Sync> {
        match self {
            /* ---------- 布林運算 ---------- */
            Expression::Or(exprs) => {
                let id = shared_album_id.clone();
                let filters = exprs;
                Box::new(move |data| {
                    filters.iter().any(|expr| {
                        let filter = expr.clone().generate_filter_hide_metadata(id.clone());
                        filter(data)
                    })
                })
            }
            Expression::And(exprs) => {
                let id = shared_album_id.clone();
                let filters = exprs;
                Box::new(move |data| {
                    filters.iter().all(|expr| {
                        let filter = expr.clone().generate_filter_hide_metadata(id.clone());
                        filter(data)
                    })
                })
            }
            Expression::Not(expr) => {
                let inner = expr.generate_filter_hide_metadata(shared_album_id);
                Box::new(move |data| !inner(data))
            }

            /* ---------- 允許的 Album 條件 ---------- */
            Expression::Album(album_id) => {
                if album_id == shared_album_id {
                    Box::new(move |data| match data {
                        AbstractData::Database(db) => db.album.contains(&album_id),
                        AbstractData::Album(_) => false,
                    })
                } else {
                    // 不是這次分享的 album_id → 一律失效
                    Box::new(|_| false)
                }
            }

            /* ---------- 必須失效的補充性條件 ---------- */
            Expression::Tag(_) | Expression::Path(_) => Box::new(|_| false),

            /* ---------- 仍允許的嵌入式 / 檔案相關條件 ---------- */
            Expression::ExtType(ext_type) => Box::new(move |data| match data {
                AbstractData::Database(db) => db.ext_type.contains(&ext_type),
                AbstractData::Album(_) => false,
            }),
            Expression::Ext(ext) => {
                let ext_lower = ext.to_ascii_lowercase();
                Box::new(move |data| match data {
                    AbstractData::Database(db) => db.ext.to_ascii_lowercase().contains(&ext_lower),
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Model(model) => {
                let model_lower = model.to_ascii_lowercase();
                Box::new(move |data| match data {
                    AbstractData::Database(db) => db
                        .exif_vec
                        .get("Model")
                        .map_or(false, |v| v.to_ascii_lowercase().contains(&model_lower)),
                    AbstractData::Album(_) => false,
                })
            }
            Expression::Make(make) => {
                let make_lower = make.to_ascii_lowercase();
                Box::new(move |data| match data {
                    AbstractData::Database(db) => db
                        .exif_vec
                        .get("Make")
                        .map_or(false, |v| v.to_ascii_lowercase().contains(&make_lower)),
                    AbstractData::Album(_) => false,
                })
            }

            /* ---------- Any：移除 tag / alias / album / path 比對 ---------- */
            Expression::Any(identifier) => {
                let any_lower = identifier.to_ascii_lowercase();
                Box::new(move |data| match data {
                    AbstractData::Database(db) => {
                        db.ext_type.contains(&identifier)
                            || db.ext.to_ascii_lowercase().contains(&any_lower)
                            || db
                                .exif_vec
                                .get("Make")
                                .map_or(false, |v| v.to_ascii_lowercase().contains(&any_lower))
                            || db
                                .exif_vec
                                .get("Model")
                                .map_or(false, |v| v.to_ascii_lowercase().contains(&any_lower))
                    }
                    AbstractData::Album(_) => false,
                })
            }
        }
    }
}
