use crate::interop::{
    equipslots::EquipSlot,
    races::{Gender, ModelRace},
};
use serde::{Deserialize, Serialize};
use std::u8;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EqdpJson {
    pub Entry: EqdpEntry,
    pub Gender: Gender,
    pub Race: ModelRace,
    pub Slot: EquipSlot,
    pub SetId: u16,
}

pub type EqdpEntry = u8;
