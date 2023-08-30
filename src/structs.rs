use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SelectType {
    Single,
    Multi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SimpleMod {
    pub Name: String,
    pub Category: String,
    pub FullPath: String,
    pub DatFile: String,
    pub ModOffset: u32,
    pub ModSize: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct OptionList {
    pub Name: String,
    pub Description: Option<String>,
    pub ImagePath: String,
    pub ModsJsons: Vec<SimpleMod>,
    pub GroupName: String,
    pub IsChecked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ModGroup {
    pub GroupName: String,
    pub SelectionType: SelectType,
    pub OptionList: Vec<OptionList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ModPackPage {
    pub PageIndex: i32,
    pub ModGroups: Vec<ModGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ExtendedModPack {
    pub Name: String,
    pub Author: Option<String>,
    pub Description: Option<String>,
    pub ModPackPages: Option<Vec<ModPackPage>>,
    pub SimpleModsList: Option<Vec<SimpleMod>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SimpleModPack {
    pub TtmpVersion: String,
    pub Name: String,
    pub Author: String,
    pub Version: String,
    pub Description: String,
    pub SimpleModsList: Option<Vec<SimpleMod>>,
}
