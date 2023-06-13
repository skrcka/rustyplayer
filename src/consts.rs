pub const MEDIA_PATH: &str = if cfg!(debug_assertions) {
    "media"
} else {
    "/media"
};

pub const RESOURCE_PATH: &str = if cfg!(debug_assertions) {
    "resource"
} else {
    "/resource"
};

pub const WEB_PATH: &str = if cfg!(debug_assertions) {
    "static"
} else {
    "/static"
};
