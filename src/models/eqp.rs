use std::u8;

use binrw::binrw;
use serde::{Deserialize, Serialize};

use crate::gamedata::equipslots::EquipSlot;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EqpJson {
    pub Slot: EquipSlot,
    pub SetId: u16,
    pub Entry: u64,
}
#[binrw]
#[br(import(cnt:u32,))]
pub struct EqpEntry {
    #[br(count = cnt)]
    pub data: Vec<u8>,
}
