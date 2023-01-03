use std::io::Cursor;

use crate::gamedata::equipslots::{BodySlot, EquipSlot};
use crate::gamedata::gendermodelrace::gr::GenderRace;
use crate::gamedata::objecttypes::ObjectType;
use crate::models::eqdp::{EqdpDiskModel, EqdpJson};
use crate::models::eqp::{EqpDiskModel, EqpJson};
use crate::models::est::{EstDisk, EstJson, EstType};
use crate::models::gmp::{GmpDiskModel, GmpJson};
use crate::models::imc::ImcEntry;
use crate::models::{gmp::GmpEntry, MetaManipulation};
use array_tool::vec::Union;
use binrw::{binrw, BinRead, NullString};
use regex::Regex;
use serde::{Deserialize, Serialize};
#[binrw]
#[brw(repr(u32))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
enum MetaType {
    Imc = 1,
    Eqdp,
    Eqp,
    Est,
    Gmp,
    Rsp,
}

#[binrw]
#[derive(Debug, Clone, Copy)]
struct MetaBlock {
    pub meta_type: MetaType,
    pub meta_offset: u32,
    pub meta_size: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct MetaFileHeader {
    version: u32,
    file_path: NullString,
    num_headers: u32,
    header_size: u32,
    header_start: u32,
    #[br(count = num_headers)]
    #[br(pad_size_to = header_size)]
    #[bw(pad_size_to = self.header_size)]
    blocks: Vec<MetaBlock>,
    #[brw(ignore)]
    typ: ObjectType,
}

#[allow(dead_code)]
impl MetaFileHeader {
    pub fn get_regex_pt(&mut self) -> ObjectType {
        let re = Regex::new(r"chara/(?P<PrimaryType>[a-z]*)/(?P<PrimaryPrefix>[a-z])(?P<PrimaryId>\d{4})(/obj/(?P<SecondaryType>[a-z]*)?/?(?P<SecondaryPrefix>[a-z])?(?P<SecondaryId>\d{4}))?/[a-z]\d{4}.(?P<Slot>[a-z]{3})?(\d{4}?)?\.meta").expect("Failed to compile regex");
        let string = self.file_path.to_string();
        let mch = re
            .captures(string.as_str())
            .unwrap()
            .name("PrimaryType")
            .unwrap()
            .as_str();
        self.typ = if mch.starts_with("chara/accessory/") {
            ObjectType::Accessory
        } else if mch.starts_with("chara/equipment/") {
            ObjectType::Equipment
        } else {
            ObjectType::Housing
        };
        return self.typ;
    }
    pub fn get_regex_st(&self) -> BodySlot {
        let re = Regex::new(r"chara/(?P<PrimaryType>[a-z]*)/(?P<PrimaryPrefix>[a-z])(?P<PrimaryId>\d{4})(/obj/(?P<SecondaryType>[a-z]*)?/?(?P<SecondaryPrefix>[a-z])?(?P<SecondaryId>\d{4}))?/[a-z]\d{4}.(?P<Slot>[a-z]{3})?(\d{4}?)?\.meta").expect("Failed to compile regex");
        let test = self.file_path.to_string();
        let bs = re
            .captures(test.as_str())
            .unwrap()
            .name("SecondaryType")
            .unwrap()
            .as_str();
        return match bs {
            "zear" => BodySlot::Zear,
            "face" => BodySlot::Face,
            "hair" => BodySlot::Hair,
            "tail" => BodySlot::Tail,
            "body" => BodySlot::Body,
            _ => BodySlot::Unknown,
        };
    }
    pub fn get_regex_pid(&self) -> String {
        let re = Regex::new(r"chara/(?P<PrimaryType>[a-z]*)/(?P<PrimaryPrefix>[a-z])(?P<PrimaryId>\d{4})(/obj/(?P<SecondaryType>[a-z]*)?/?(?P<SecondaryPrefix>[a-z])?(?P<SecondaryId>\d{4}))?/[a-z]\d{4}.(?P<Slot>[a-z]{3})?(\d{4}?)?\.meta").expect("Failed to compile regex");
        re.captures(self.file_path.to_string().as_str())
            .unwrap()
            .name("PrimaryId")
            .unwrap()
            .as_str()
            .to_string()
    }
    pub fn get_regex_sid(&self) -> String {
        let re = Regex::new(r"chara/(?P<PrimaryType>[a-z]*)/(?P<PrimaryPrefix>[a-z])(?P<PrimaryId>\d{4})(/obj/(?P<SecondaryType>[a-z]*)?/?(?P<SecondaryPrefix>[a-z])?(?P<SecondaryId>\d{4}))?/[a-z]\d{4}.(?P<Slot>[a-z]{3})?(\d{4}?)?\.meta").expect("Failed to compile regex");
        re.captures(self.file_path.to_string().as_str())
            .unwrap()
            .name("PrimaryId")
            .unwrap()
            .as_str()
            .to_string()
    }
    pub fn get_regex_slot(&self) -> EquipSlot {
        let re = Regex::new(r"chara/(?P<PrimaryType>[a-z]*)/(?P<PrimaryPrefix>[a-z])(?P<PrimaryId>\d{4})(/obj/(?P<SecondaryType>[a-z]*)?/?(?P<SecondaryPrefix>[a-z])?(?P<SecondaryId>\d{4}))?/[a-z]\d{4}.(?P<Slot>[a-z]{3})?(\d{4}?)?\.meta").expect("Failed to compile regex");
        let slot = re
            .captures(self.file_path.to_string().as_str())
            .expect("Unable to get captures")
            .name("Slot")
            .expect("Unable to get slot")
            .as_str()
            .to_string();
        match slot.as_str() {
            "glv" => EquipSlot::Hands,
            "top" => EquipSlot::Body,
            "dwn" => EquipSlot::Legs,
            "sho" => EquipSlot::Feet,
            "met" => EquipSlot::Head,
            _ => panic!("unknown slot"),
        }
    }
    pub fn parse_meta_blocks(&self, mut stream: Cursor<Vec<u8>>) -> Vec<MetaManipulation> {
        let mut retr: Vec<MetaManipulation> = Vec::new();
        for block in self.blocks.clone() {
            stream.set_position(block.meta_offset as u64);
            let meta_manipulation = match block.meta_type {
                MetaType::Eqp => {
                    stream.set_position(block.meta_offset as u64);
                    let slot = self.get_regex_slot();
                    let (byts, offst) = slot.eqp_bytes_offset();
                    let mut num: u64 = 0;
                    let eqp_data = EqpDiskModel::read_le_args(&mut stream, (byts,))
                        .expect("Error reading eqp disk model");
                    for i in 0..eqp_data.data.len() {
                        num |= (eqp_data.data[i] as u64) << offst + (i as u32) * 8
                    }
                    let mut meta = Vec::new();
                    meta.push(MetaManipulation::Eqp(EqpJson {
                        Slot: slot,
                        SetId: self
                            .get_regex_pid()
                            .parse::<u16>()
                            .expect("Failed to parse primary id"),
                        Entry: num & slot.eqp_mask(),
                    }));
                    meta
                }
                MetaType::Eqdp => {
                    let mut ret = Vec::new();
                    let num = block.meta_size / 5;
                    stream.set_position(block.meta_offset as u64);
                    for _ in 1..num {
                        let gr = GenderRace::read_le(&mut stream).expect("Error reading gr");
                        let bv =
                            EqdpDiskModel::read(&mut stream).expect("Error reading bv in eqdp");
                        let slot = self.get_regex_slot();
                        let mut entry = 0;
                        if bv & 1 == 1 {
                            entry |= 1 << slot.eqdp_bytes_offset();
                        }
                        if bv & 2 == 2 {
                            entry |= 1 << (slot.eqdp_bytes_offset() + 1);
                        }
                        entry = entry & slot.eqdp_mask() as u8;
                        ret.push(MetaManipulation::Eqdp(EqdpJson {
                            Entry: entry,
                            Gender: gr.split().0,
                            Race: gr.split().1,
                            Slot: slot,
                            SetId: self
                                .get_regex_pid()
                                .parse::<u16>()
                                .expect("error parsing SetId"),
                        }));
                    }
                    ret
                }
                MetaType::Imc => {
                    let mut ret = Vec::new();
                    let num = block.meta_size / 6;
                    for _ in 0..num {
                        let ent = ImcEntry::read(&mut stream);
                    }
                    ret
                }
                MetaType::Est => {
                    let mut retr = Vec::new();
                    stream.set_position(block.meta_offset as u64);
                    let num = block.meta_size / 6;
                    for _ in 1..num {
                        let estd = EstDisk::read(&mut stream).expect("Error decoding est");
                        let slot = match (self.get_regex_st(), self.get_regex_slot()) {
                            (BodySlot::Face, _) => EstType::Face,
                            (BodySlot::Hair, _) => EstType::Hair,
                            (_, EquipSlot::Head) => EstType::Head,
                            (_, EquipSlot::Body) => EstType::Body,
                            (_, _) => unimplemented!(),
                        };
                        let gnr: GenderRace =
                            num::FromPrimitive::from_u16(estd.gr).expect("Error reading gr");
                        retr.push(MetaManipulation::Est(EstJson {
                            Gender: gnr.split().0,
                            Race: gnr.split().1,
                            Slot: slot,
                            Entry: estd.val,
                            SetId: estd.id,
                        }));
                    }
                    retr
                }
                MetaType::Gmp => {
                    stream.set_position(block.meta_offset as u64);
                    let gmp =
                        GmpDiskModel::read_le(&mut stream).expect("Error reading gmp disk model");
                    let mut meta = Vec::new();
                    meta.push(MetaManipulation::Gmp(GmpJson {
                        SetId: self
                            .get_regex_pid()
                            .parse::<u16>()
                            .expect("Failed to parse primary id"),
                        Entry: GmpEntry {
                            Enabled: (gmp.val & 1) == 1,
                            Animated: (gmp.val & 2) == 1,
                            RotationA: ((gmp.val >> 2) & 0x3FF) as u16,
                            RotationB: ((gmp.val >> 12) & 0x3FF) as u16,
                            RotationC: ((gmp.val >> 22) & 0x3FF) as u16,
                            UnknownA: (gmp.unknown_total & 0x0F) as u8,
                            UnknownB: ((gmp.unknown_total >> 4) & 0x0F) as u8,
                            Value: (gmp.val as u64 & !(0xFF00000000))
                                | ((gmp.unknown_total as u64 & 0xFF) << 32),
                            UnknownTotal: gmp.unknown_total,
                        },
                    }));
                    meta
                }
                MetaType::Rsp => Vec::new(),
            };
            retr = retr.union(meta_manipulation);
        }
        return retr;
    }
}
