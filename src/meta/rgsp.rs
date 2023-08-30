use binrw::binrw;

use crate::interop::races::{Gender, Subrace};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct RgspHeader {
    pub flag: u8,
    #[br(if(flag == 0xFF, 1))]
    pub ver: u16,
    #[br(map(|x: u8| Subrace::from_id(x+1)))]
    pub sub_race: Subrace,
    pub gender: Gender,
    #[br(if(gender == Gender::Male))]
    pub male: Option<RgspMale>,
    #[br(if(gender == Gender::Female))]
    pub female: Option<RgspFemale>,
}
#[binrw]
#[brw(little)]
#[derive(Debug, Default)]
pub struct RgspMale {
    pub male_min_size: f32,
    pub male_max_size: f32,
    pub male_min_tail: f32,
    pub male_max_tail: f32,
}
#[binrw]
#[brw(little)]
#[derive(Debug, Default)]
pub struct RgspFemale {
    pub female_min_size: f32,
    pub female_max_size: f32,
    pub female_min_tail: f32,
    pub female_max_tail: f32,
    pub bust_min_x: f32,
    pub bust_min_y: f32,
    pub bust_min_z: f32,
    pub bust_max_x: f32,
    pub bust_max_y: f32,
    pub bust_max_z: f32,
}
