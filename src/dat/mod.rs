pub mod datfile;
mod model;
mod shared;
mod standard;
mod tex;

use crate::constants::MAX_LODS;
use binrw::BinRead;

#[derive(Debug, Clone, Copy, BinRead)]
#[br(little)]
pub struct SectionInfo<T: for<'a> BinRead<Args<'a> = ()> + Default + Copy> {
    pub stack_size: T,
    pub runtime_size: T,
    pub vertex: [T; MAX_LODS],
    pub edge: [T; MAX_LODS],
    pub index: [T; MAX_LODS],
}
