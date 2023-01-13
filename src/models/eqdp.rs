use serde::{Deserialize, Serialize};
use std::u8;

use crate::gamedata::{
    equipslots::EquipSlot,
    gendermodelrace::{Gender, ModelRace},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EqdpJson {
    pub Entry: u8,
    pub Gender: Gender,
    pub Race: ModelRace,
    pub Slot: EquipSlot,
    pub SetId: u16,
}

pub type EqdpDisk = u8;
