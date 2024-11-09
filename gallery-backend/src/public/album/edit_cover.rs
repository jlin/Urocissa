use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::public::{
    abstract_data::AbstractData,
    database_struct::database::{self, definition::DataBase},
    tree::TREE,
};

use super::Album;

impl Album {
    pub fn set_cover(self: &mut Self, cover_data: &DataBase) {
        self.cover = Some(cover_data.hash);
        self.width = cover_data.width;
        self.height = cover_data.height;
    }
    pub fn auto_set_cover(self: &mut Self) {
        let ref_data = TREE.in_memory.read().unwrap();
        let data_in_album: Vec<&DataBase> = ref_data
            .par_iter()
            .filter_map(
                |database_timestamp| match &database_timestamp.abstract_data {
                    AbstractData::DataBase(database) => {
                        if database.album.contains(&self.id) {
                            Some(database)
                        } else {
                            None
                        }
                    }
                    AbstractData::Album(_) => None,
                },
            )
            .collect();
        if let Some(first_database) = data_in_album.first() {
            self.set_cover(first_database);
        }
    }
}
