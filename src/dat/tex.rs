use binrw::binrw;

#[derive(Debug, Clone, Copy)]
#[binrw]
#[br(little)]
pub struct LodBlock {
    pub compressed_offset: u32,
    pub compressed_size: u32,
    pub decompressed_size: u32,
    pub block_offset: u32,
    pub block_count: u32,
}

#[derive(Debug, Clone)]
#[binrw]
pub struct TextureBlock {
    #[br(pad_before = 8)]
    pub num_blocks: u32,
    #[br(count = num_blocks)]
    pub lods: Vec<LodBlock>,
}
