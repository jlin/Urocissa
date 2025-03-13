pub const ROW_BATCH_NUMBER: usize = 20;

pub const PROCESS_BATCH_NUMBER: usize = 100;

pub const SHOULD_SWAP_WIDTH_HEIGHT_ROTATION: [&str; 4] = ["90", "-90", "270", "-270"];

pub const VALID_IMAGE_EXTENSIONS: [&str; 9] = [
    "jpg", "jpeg", "jfif", "jpe", "png", "tif", "tiff", "webp", "bmp",
];

pub const VALID_VIDEO_EXTENSIONS: [&str; 9] = [
    "gif", "mp4", "webm", "mkv", "mov", "avi", "flv", "wmv", "mpeg",
];

pub const DEFAULT_PRIORITY_LIST: [&str; 4] =
    ["DateTimeOriginal", "filename", "modified", "scan_time"];
