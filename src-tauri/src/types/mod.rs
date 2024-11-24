mod archive_format;
mod check_update_result;
mod comic;
mod proxy_mode;
mod web_qrcode_data;

pub use archive_format::*;
pub use check_update_result::*;
pub use comic::*;
pub use proxy_mode::*;
pub use web_qrcode_data::*;

pub type AsyncRwLock<T> = tokio::sync::RwLock<T>;
