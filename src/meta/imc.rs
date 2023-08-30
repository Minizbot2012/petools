use crate::interop::{
    equipslots::{BodySlot, EquipSlot},
    objecttypes::ObjectType,
};
use binrw::binread;
use serde::{Deserialize, Serialize};

#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ImcEntry {
    pub MaterialId: u8,
    pub DecalId: u8,
    #[br(temp)]
    pub AttributeAndSound: u16,
    pub VfxId: u8,
    pub MaterialAnimationId: u8,
    #[br(calc=AttributeAndSound&0x3FF)]
    pub AttributeMask: u16,
    #[br(calc=(AttributeAndSound >> 10) as u8)]
    pub SoundId: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ImcJson {
    pub PrimaryId: u32,
    pub Variant: u32,
    pub SecondaryId: u32,
    pub ObjectType: ObjectType,
    pub EquipSlot: EquipSlot,
    pub BodySlot: BodySlot,
    pub Entry: ImcEntry,
}
