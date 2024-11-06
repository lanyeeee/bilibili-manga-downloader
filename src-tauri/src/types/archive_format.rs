use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub enum ArchiveFormat {
    #[default]
    Image,
    Zip,
    Cbz,
}
