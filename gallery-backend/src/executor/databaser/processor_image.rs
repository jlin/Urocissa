use std::{collections::BTreeMap, error::Error, io, path::Path};

use anyhow::Context;

use crate::public::database_struct::{database::definition::DataBase, hash_alias::HashAliasSize};

pub fn process_image_info(hash_alias_size: HashAliasSize) -> DataBase {
    let mut hash_alias = hash_alias_size.hash_alias;
    let source_path = hash_alias.source_path();
    let mut exif_tuple = BTreeMap::new();
    if let Ok(exif) = read_exif(&source_path) {
        for field in exif.fields() {
            let tag = field.tag.to_string();
            let value = field.display_value().with_unit(&exif).to_string();
            let ifd_num = field.ifd_num;
            if exif_tuple.get(&tag).is_some() {
                // Only replace if the new field is from the PRIMARY IFD
                if ifd_num == exif::In::PRIMARY {
                    exif_tuple.insert(tag, value);
                }
            } else {
                // If the key doesn't exist, insert it as usual
                exif_tuple.insert(tag, value);
            }
        }
    }
    return DataBase::new(
        std::mem::take(&mut hash_alias.hash),
        hash_alias_size.size,
        hash_alias.ext(),
        std::mem::take(&mut exif_tuple),
        std::mem::take(&mut hash_alias.alias.alias),
    );
}

fn read_exif(file_path: &Path) -> Result<exif::Exif, Box<dyn Error>> {
    let exif_reader = exif::Reader::new();
    let file = std::fs::File::open(file_path)
        .with_context(|| format!("read_exif: Failed to open file {:?}", file_path))?;
    let mut bufreader = io::BufReader::with_capacity(1024 * 1024, &file);
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .with_context(|| format!("read_exif: Failed to read exif of file {:?}", file_path))?;
    Ok(exif)
}
