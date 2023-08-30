use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EquipSlot {
    Unknown,
    MainHand,
    OffHand,
    Head,
    Body,
    Hands,
    Belt,
    Legs,
    Feet,
    Ears,
    Neck,
    Wrists,
    RFinger,
    BothHand,
    LFinger,
    HeadBody,
    BodyHandsLegsFeet,
    SoulCrystal,
    LegsFeet,
    FullBody,
    BodyHands,
    BodyLegsFeet,
    ChestHands,
    Nothing,
    All,
}

impl EquipSlot {
    #[allow(dead_code)]
    pub fn to_suffix(self) -> &'static str {
        match self {
            EquipSlot::Head => "met",
            EquipSlot::Hands => "glv",
            EquipSlot::Legs => "dwn",
            EquipSlot::Feet => "sho",
            EquipSlot::Body => "top",
            EquipSlot::Ears => "ear",
            EquipSlot::Neck => "nek",
            EquipSlot::RFinger => "rir",
            EquipSlot::LFinger => "ril",
            EquipSlot::Wrists => "wrs",
            _ => "unk",
        }
    }
    pub fn from_suffix(st: &str) -> EquipSlot {
        match st {
            "met" => EquipSlot::Head,
            "glv" => EquipSlot::Hands,
            "dwn" => EquipSlot::Legs,
            "sho" => EquipSlot::Feet,
            "top" => EquipSlot::Body,
            "ear" => EquipSlot::Ears,
            "nek" => EquipSlot::Neck,
            "rir" => EquipSlot::RFinger,
            "ril" => EquipSlot::LFinger,
            "wrs" => EquipSlot::Wrists,
            _ => EquipSlot::Unknown,
        }
    }
    pub fn eqp_bytes_offset(self) -> (u32, u32) {
        match self {
            EquipSlot::Body => (2, 0),
            EquipSlot::Legs => (1, 2),
            EquipSlot::Hands => (1, 3),
            EquipSlot::Feet => (1, 4),
            EquipSlot::Head => (3, 5),
            _ => panic!("error bad slot for eqp"),
        }
    }
    pub fn eqp_mask(self) -> u64 {
        match self {
            EquipSlot::Body => 0xFFFF,
            EquipSlot::Legs => 0xFF << 16,
            EquipSlot::Hands => 0xFF << 24,
            EquipSlot::Feet => 0xFF << 32,
            EquipSlot::Head => 0xFF << 40,
            _ => panic!("error bad slot for eqp"),
        }
    }
    pub fn eqdp_mask(self) -> u8 {
        match self {
            EquipSlot::Head => 0b0000000011,
            EquipSlot::Body => 0b0000001100,
            EquipSlot::Hands => 0b0000110000,
            EquipSlot::Legs => 0b0011000000,
            EquipSlot::Feet => 0b11000000,
            EquipSlot::Ears => 0b0000000011,
            EquipSlot::Neck => 0b0000001100,
            EquipSlot::Wrists => 0b0000110000,
            EquipSlot::RFinger => 0b0011000000,
            EquipSlot::LFinger => 0b11000000,
            _ => panic!("Invalid Slot for eqdp"),
        }
    }
    pub fn eqdp_bytes_offset(self) -> u64 {
        match self {
            EquipSlot::Head => 0,
            EquipSlot::Body => 2,
            EquipSlot::Hands => 4,
            EquipSlot::Legs => 6,
            EquipSlot::Feet => 8,
            EquipSlot::Ears => 0,
            EquipSlot::Neck => 2,
            EquipSlot::Wrists => 4,
            EquipSlot::RFinger => 6,
            EquipSlot::LFinger => 8,
            _ => panic!("Invalid slot of eqdp"),
        }
    }
    #[allow(dead_code)]
    pub fn is_accessory(self) -> bool {
        return match self {
            EquipSlot::Ears => true,
            EquipSlot::Neck => true,
            EquipSlot::Wrists => true,
            EquipSlot::LFinger => true,
            EquipSlot::RFinger => true,
            _ => false,
        };
    }
    #[allow(dead_code)]
    pub fn is_equipment(self) -> bool {
        return match self {
            EquipSlot::Head => true,
            EquipSlot::Legs => true,
            EquipSlot::Feet => true,
            EquipSlot::Hands => true,
            EquipSlot::Body => true,
            _ => false,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, IntoStaticStr, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum BodySlot {
    Unknown,
    Hair,
    Face,
    Tail,
    Body,
    Zear,
}
