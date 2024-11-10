use std::collections::HashSet;

use arrayvec::ArrayString;
use serde::{Deserialize, Serialize};

use super::{album::Album, database_struct::database::definition::DataBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbstractData {
    DataBase(DataBase),
    Album(Album),
}

impl AbstractData {
    pub fn compute_timestamp(self: &Self, priority_list: &Vec<&str>) -> u128 {
        match self {
            AbstractData::DataBase(database) => database.compute_timestamp(priority_list),
            AbstractData::Album(album) => album.created_time,
        }
    }
    pub fn hash(self: &Self) -> ArrayString<64> {
        match self {
            AbstractData::DataBase(database) => database.hash,
            AbstractData::Album(album) => album.id,
        }
    }
    pub fn width(self: &Self) -> u32 {
        match self {
            AbstractData::DataBase(database) => database.width,
            AbstractData::Album(album) => 300,
        }
    }
    pub fn height(self: &Self) -> u32 {
        match self {
            AbstractData::DataBase(database) => database.height,
            AbstractData::Album(album) => 300,
        }
    }
    pub fn tag(self: &Self) -> &HashSet<String> {
        match self {
            AbstractData::DataBase(database) => &database.tag,
            AbstractData::Album(album) => &album.tag,
        }
    }
}
