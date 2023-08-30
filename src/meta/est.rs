use crate::interop::{
    characterutils,
    races::{Gender, ModelRace},
};
use binrw::binread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EstType {
    Hair = characterutils::Index::HairEst as isize,
    Face = characterutils::Index::FaceEst as isize,
    Body = characterutils::Index::BodyEst as isize,
    Head = characterutils::Index::HeadEst as isize,
}

#[binread]
#[br(little)]
pub struct EstEntry {
    pub gr: u16,
    pub id: u16,
    pub val: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct EstJson {
    pub Slot: EstType,
    pub Gender: Gender,
    pub Race: ModelRace,
    pub Entry: u16,
    pub SetId: u16,
}
