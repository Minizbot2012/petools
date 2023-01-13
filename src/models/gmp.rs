use binrw::binread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GmpJson {
    pub Entry: GmpEntry,
    pub SetId: u16,
}

#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GmpEntry {
    #[br(temp)]
    pub val: u32,
    pub unknown_total: u8,
    #[br(calc = (val&1)==1)]
    pub Enabled: bool,
    #[br(calc = (val&2)==1)]
    pub Animated: bool,
    #[br(calc = (val as u64 & !0xFF00000000)|((unknown_total as u64 & 0xFF) << 32))]
    pub Value: u64,
    #[br(calc = ((val >> 2) & 0x3FF) as u16)]
    pub RotationA: u16,
    #[br(calc = ((val >> 12) & 0x3FF) as u16)]
    pub RotationB: u16,
    #[br(calc = ((val >> 22) & 0x3FF) as u16)]
    pub RotationC: u16,
    #[br(calc = (unknown_total & 0x0F) as u8)]
    pub UnknownA: u8,
    #[br(calc = ((unknown_total >> 4) & 0x0F) as u8)]
    pub UnknownB: u8,
}
