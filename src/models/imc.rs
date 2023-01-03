use binrw::binrw;

#[binrw]
#[brw(little)]
#[allow(non_snake_case)]
pub struct ImcEntry {
    pub MaterialId: u8,
    pub DecalId: u8,
    pub AttributeAndSound: u16,
    pub VfxId: u8,
    pub MaterialAnimationId: u8,
    #[br(calc=AttributeAndSound&0x3FFF)]
    pub AttributeMask: u16,
    #[br(calc=(AttributeAndSound >> 10) as u8)]
    pub SoundId: u8,
}
