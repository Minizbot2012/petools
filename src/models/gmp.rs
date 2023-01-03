use binrw::binrw;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GmpEntry {
    pub Enabled: bool,
    pub Animated: bool,
    pub Value: u64,
    pub UnknownTotal: u8,
    pub RotationA: u16,
    pub RotationB: u16,
    pub RotationC: u16,
    pub UnknownA: u8,
    pub UnknownB: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GmpJson {
    pub Entry: GmpEntry,
    pub SetId: u16,
}

#[binrw]
pub struct GmpDiskModel {
    pub val: u32,
    pub unknown_total: u8,
}
