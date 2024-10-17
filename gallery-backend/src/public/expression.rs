use serde::{Deserialize, Serialize};

use super::database_struct::database::definition::DataBase;

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
    Any(String),
}

impl Expression {
    pub fn generate_filter(self) -> Box<dyn Fn(&DataBase) -> bool + Sync + Send> {
        match self {
            Expression::Or(expressions) => {
                let filters = expressions
                    .into_iter()
                    .map(|e| e.generate_filter())
                    .collect::<Vec<_>>();
                Box::new(move |data: &DataBase| filters.iter().any(|filter| filter(data)))
            }
            Expression::And(expressions) => {
                let filters = expressions
                    .into_iter()
                    .map(|e| e.generate_filter())
                    .collect::<Vec<_>>();
                Box::new(move |data: &DataBase| filters.iter().all(|filter| filter(data)))
            }
            Expression::Not(expression) => {
                let filter = expression.generate_filter();
                Box::new(move |data: &DataBase| !filter(data))
            }
            Expression::Tag(tag) => Box::new(move |data: &DataBase| data.tag.contains(&tag)),
            Expression::ExtType(ext_type) => {
                Box::new(move |data: &DataBase| data.ext_type.contains(&ext_type))
            }
            Expression::Ext(ext) => {
                let ext = ext.to_ascii_lowercase();
                Box::new(move |data: &DataBase| data.ext.to_ascii_lowercase().contains(&ext))
            }
            Expression::Model(model) => {
                let model = model.to_ascii_lowercase();
                Box::new(move |data: &DataBase| match data.exif_vec.get("Model") {
                    Some(model_of_exif) => model_of_exif.to_ascii_lowercase().contains(&model),
                    None => false,
                })
            }
            Expression::Make(make) => {
                let make = make.to_ascii_lowercase();
                Box::new(move |data: &DataBase| match data.exif_vec.get("Make") {
                    Some(make_of_exif) => make_of_exif.to_ascii_lowercase().contains(&make),
                    None => false,
                })
            }
            Expression::Path(path) => {
                let path = path.to_ascii_lowercase();
                Box::new(move |data: &DataBase| {
                    data.alias
                        .iter()
                        .any(|file_modify| file_modify.file.to_ascii_lowercase().contains(&path))
                })
            }
            Expression::Any(any_identifier) => {
                let any_identifier_lowercase = any_identifier.to_ascii_lowercase();
                Box::new(move |data: &DataBase| {
                    let tag_match = data.tag.contains(&any_identifier);
                    let ext_type_match = data.ext_type.contains(&any_identifier);
                    let ext_match = data
                        .ext
                        .to_ascii_lowercase()
                        .contains(&any_identifier_lowercase);
                    let make_match = data.exif_vec.get("Make").map_or(false, |make_of_exif| {
                        make_of_exif
                            .to_ascii_lowercase()
                            .contains(&any_identifier_lowercase)
                    });
                    let model_match = data.exif_vec.get("Model").map_or(false, |model_of_exif| {
                        model_of_exif
                            .to_ascii_lowercase()
                            .contains(&any_identifier_lowercase)
                    });
                    let path_match = data.alias.iter().any(|file_modify| {
                        file_modify
                            .file
                            .to_ascii_lowercase()
                            .contains(&any_identifier_lowercase)
                    });
                    tag_match
                        || ext_type_match
                        || ext_match
                        || make_match
                        || model_match
                        || path_match
                })
            }
        }
    }
}
