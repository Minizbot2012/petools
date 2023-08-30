use super::SectionInfo;
use binrw::BinRead;

#[derive(Debug, Clone, Copy, BinRead)]
#[br(little)]
#[allow(dead_code)]
pub struct ModelBlock {
    pub num_blocks: u32,
    pub num_used_blocks: u32,
    pub version: u32,
    pub uncompressed_size: SectionInfo<u32>,
    pub compressed_size: SectionInfo<u32>,
    pub offset: SectionInfo<u32>,
    pub index: SectionInfo<u16>,
    pub num: SectionInfo<u16>,
    pub vertex_dec_num: u16,
    pub material_num: u16,
    pub num_lods: u8,
    #[br(map = | x: u8 | x!=0)]
    pub index_buffer_streaming_enabled: bool,
    #[br(map = | x: u8 | x!=0)]
    #[br(pad_after = 1)]
    pub edge_geometry_enabled: bool,
}
