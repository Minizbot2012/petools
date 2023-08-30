use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString)]
pub enum ObjectType {
    Unknown,
    Vfx,
    DemiHuman,
    Accessory,
    World,
    Housing,
    Monster,
    Icon,
    LoadingScreen,
    Map,
    Interface,
    Equipment,
    Character,
    Weapon,
    Font,
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Unknown
    }
}
