use crate::{
    public::constant::VALID_IMAGE_EXTENSIONS,
    public::structure::database_struct::{database::definition::Database, file_modify::FileModify},
};
use anyhow::Context;
use arrayvec::ArrayString;
use blake3::Hasher;
use std::{
    collections::{BTreeMap, HashSet},
    fs::{File, metadata},
    io::Read,
    path::Path,
    time::UNIX_EPOCH,
};

impl Database {
    /// Constructs a fresh [`Database`] from the file at `path`.
    ///
    /// # What is **fully** initialised
    /// | Field     | Source of value                                        |
    /// |-----------|--------------------------------------------------------|
    /// | `hash`    | 64-char hex digest via **BLAKE3** (`blake3_hasher`)    |
    /// | `size`    | `std::fs::metadata(path)?.len()`                       |
    /// | `ext`     | Normalised (ASCII-lower-case) file extension           |
    /// | `ext_type`| `"image"` or `"video"` via `determine_type`            |
    /// | `alias`   | Single [`FileModify`] describing `path`                |
    /// | `pending` | `false`                                                |
    ///
    /// # What remains **unset** (`default()` / `Vec::new()`)
    /// `width`, `height`, `thumbhash`, `phash`, `exif_vec`, `tag`, `album`.
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        // --- 1 · extension -------------------------------------------------
        let ext = path
            .extension()
            .ok_or_else(|| anyhow::anyhow!("File has no extension: {:?}", path))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Extension is not valid UTF-8: {:?}", path))?
            .to_ascii_lowercase(); // ASCII-only, cheap copy :contentReference[oaicite:2]{index=2}

        // --- 2 · file metadata --------------------------------------------
        let md = metadata(path).with_context(|| format!("Failed to read metadata: {:?}", path))?; // :contentReference[oaicite:3]{index=3}
        let size = md.len();

        let modified_millis = md
            .modified()?
            .duration_since(UNIX_EPOCH)
            .with_context(|| format!("Modification time is before UNIX_EPOCH: {:?}", path))?
            .as_millis();

        // --- 3 · compute BLAKE3 digest ------------------------------------
        let hash = blake3_hasher(path)?; // fast parallel hash :contentReference[oaicite:4]{index=4}

        // --- 4 · first alias entry ----------------------------------------
        let file_modify = FileModify::new(path, modified_millis);

        // --- 5 · assemble struct ------------------------------------------
        Ok(Self {
            hash,
            size,
            width: 0,
            height: 0,
            thumbhash: Vec::new(),
            phash: Vec::new(),
            ext_type: Self::determine_type(&ext),
            ext,
            exif_vec: BTreeMap::new(),
            tag: HashSet::new(),
            album: HashSet::new(),
            alias: vec![file_modify],
            pending: false,
        })
    }

    fn determine_type(ext: &str) -> String {
        if VALID_IMAGE_EXTENSIONS.contains(&ext) {
            "image"
        } else {
            "video"
        }
        .into()
    }
}

/// Helper: stream the file and return 64-character hex digest.
fn blake3_hasher(file_path: &Path) -> anyhow::Result<ArrayString<64>> {
    let mut file =
        File::open(file_path).with_context(|| format!("Failed to open file: {:?}", file_path))?;

    let mut hasher = Hasher::new(); // :contentReference[oaicite:5]{index=5}
    let mut buffer = [0u8; 512 * 1024];

    loop {
        let n = file
            .read(&mut buffer)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher.finalize().to_hex())
}
