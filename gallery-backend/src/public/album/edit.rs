use std::time::{SystemTime, UNIX_EPOCH};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::public::{
    abstract_data::AbstractData, database_struct::database::definition::DataBase, tree::TREE,
};

use super::Album;

impl Album {
    pub fn set_cover(&mut self, cover_data: &DataBase) {
        self.cover = Some(cover_data.hash);
        self.width = cover_data.width;
        self.height = cover_data.height;
    }

    pub fn self_update(&mut self) {
        // Acquire a read lock on the in-memory tree
        let ref_data = TREE.in_memory.read().unwrap();

        // Collect relevant DataBase entries along with their timestamps
        let mut data_in_album: Vec<(&DataBase, u128)> = ref_data
            .par_iter()
            .filter_map(
                |database_timestamp| match &database_timestamp.abstract_data {
                    AbstractData::DataBase(database) => {
                        if database.album.contains(&self.id) {
                            Some((database, database_timestamp.timestamp))
                        } else {
                            None
                        }
                    }
                    AbstractData::Album(_) => None,
                },
            )
            .collect();

        // If there are no items in the album, there's nothing to set
        if data_in_album.is_empty() {
            self.item_count = 0;
            self.item_size = 0;
            self.last_modified_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            self.start_time = None;
            self.end_time = None;
            self.cover = None;
            self.width = 0;
            self.height = 0;
            return;
        }

        // Sort the data by timestamp to determine start and end times
        data_in_album.sort_unstable_by_key(|&(_, timestamp)| timestamp);

        // Update item_count and item_size
        self.item_count = data_in_album.len();
        self.item_size = data_in_album.par_iter().map(|(db, _)| db.size).sum();

        // Update the last modified time to the current time
        self.last_modified_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Set the cover using the first (earliest) database entry
        if let Some((_, first_timestamp)) = data_in_album.first() {
            self.start_time = Some(*first_timestamp);
        }

        // Set the end_time using the last (latest) timestamp
        if let Some((last_database, last_timestamp)) = data_in_album.last() {
            if self.cover.is_none() || {
                // check whether the cover is a member of this album
                let cover = self.cover.as_ref();
                !data_in_album
                    .par_iter()
                    .any(|(database, _)| cover == Some(&database.hash))
            } {
                self.set_cover(last_database);
            }
            self.end_time = Some(*last_timestamp);
        }
    }
}
