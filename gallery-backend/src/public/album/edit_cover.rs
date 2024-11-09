use crate::public::database_struct::database::definition::DataBase;

use super::Album;

impl Album {
    pub fn set_cover(self: &mut Self, cover_data: &DataBase) {
        self.cover = Some(cover_data.hash);
        self.width = cover_data.width;
        self.height = cover_data.height;
    }
}
