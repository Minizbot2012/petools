use super::{model::ModelBlock, standard::StandardBlock, tex::TextureBlock};
use binrw::{binrw, BinRead};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[binrw]
#[brw(repr = u32)]
/// The file type of the data entry.
pub enum FileType {
    /// Empty entry, usually invalid.
    Empty = 1,
    /// Encompasses every file that is not a model or a texture, which are stored in a special fashion.
    Standard,
    /// Model (MDL) files.
    Model,
    /// Texture (TEX) files.
    Texture,
}

#[derive(Debug, Clone, Copy)]
#[binrw]
#[br(import { x : u32, y : u32 })]
#[br(map = | _ : i32 | if x != 32000 { CompressionMode::Compressed{ compressed_length : x, decompressed_length : y} } else { CompressionMode::Uncompressed { file_size : y } } )]
pub enum CompressionMode {
    // we manually map here, because for this case the enum value is also a raw value we want to extract :-)
    Compressed {
        compressed_length: u32,
        decompressed_length: u32,
    },
    Uncompressed {
        file_size: u32,
    },
}

#[derive(Debug, Clone, BinRead)]
#[br(little)]
#[allow(dead_code)]
pub struct FileInfo {
    pub size: u32,
    pub file_type: FileType,
    pub file_size: u32,
    #[br(if(file_type == FileType::Texture))]
    pub texture_block: Option<TextureBlock>,
    #[br(if(file_type == FileType::Standard))]
    pub file_block: Option<StandardBlock>,
    #[br(if(file_type == FileType::Model))]
    pub model_block: Option<ModelBlock>,
}

#[binrw::binread]
#[derive(Debug, Clone, Copy)]
#[br(little)]
pub struct DatBlockHeader {
    pub _size: u32,
    #[br(pad_before = 4)]
    #[br(temp)]
    x: u32,
    #[br(temp)]
    y: u32,
    #[br(args {x, y})]
    #[br(restore_position)]
    pub compression: CompressionMode,
}
