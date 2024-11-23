use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub enum ProxyMode {
    #[default]
    NoProxy,
    System,
    Custom,
}
