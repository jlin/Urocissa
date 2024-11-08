use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::public::{
    abstract_data::AbstractData, database_struct::database::definition::DataBase, tree::TREE,
};

use super::Album;

impl Album {
    pub fn set_cover(self: &mut Self, cover_data: &DataBase) {
        self.cover = Some(cover_data.hash);
        self.width = cover_data.width;
        self.height = cover_data.height;
    }
}
