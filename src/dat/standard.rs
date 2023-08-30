use binrw::binrw;

#[derive(Debug, Clone)]
#[binrw]
pub struct StdFileBlock {
    pub offset: u32,
    pub compressed_size: u16,
    pub uncompressed_size: u16,
}

#[derive(Debug, Clone)]
#[binrw]
pub struct StandardBlock {
    #[br(pad_before = 8)]
    pub num_blocks: u32,
    #[br(count = num_blocks)]
    pub blocks: Vec<StdFileBlock>,
}
