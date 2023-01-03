use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelRace {
    Unknown,
    Midlander,
    Highlander,
    Elezen,
    Lalafell,
    Miqote,
    Roegadyn,
    AuRa,
    Hrothgar,
    Viera,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
/// Gender of the character.
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
/// The race's "subrace". Each race has two subraces, which are actually identical (even down to the ids!)
/// with the exception of Hyurs, which have two unique subraces that are really two separate races.
pub enum Subrace {
    Midlander,
    Highlander,
    Wildwood,
    Duskwight,
    Plainsfolk,
    Dunesfolk,
    Seeker,
    Keeper,
    SeaWolf,
    Hellsguard,
    Raen,
    Xaela,
    Hellion,
    Lost,
    Rava,
    Veena,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
/// The major races of Eorzea.
pub enum Race {
    Hyur,
    Elezen,
    Lalafell,
    Miqote,
    Roegadyn,
    AuRa,
    Hrothgar,
    Viera,
}

pub mod gr {
    use binrw::binrw;
    use num_derive::FromPrimitive;
    use crate::define_race_enum;
    use paste::paste;
    use crate::gamedata::gendermodelrace::Gender;
    use crate::gamedata::gendermodelrace::ModelRace;
    use crate::gamedata::gendermodelrace::Race;
    use crate::gamedata::gendermodelrace::Subrace;
    use crate::gamedata::gendermodelrace::Gender::*;
    use crate::gamedata::gendermodelrace::Race::*;
    use crate::gamedata::gendermodelrace::Subrace::*;
    define_race_enum! {
        pub enum GenderRace {
            [101](Hyur, Male, Midlander),
            [201](Hyur, Female, Midlander),
            [301](Hyur, Male, Highlander),
            [401](Hyur, Female, Highlander),

            [501](Elezen, Male),
            [601](Elezen, Female),

            [701](Miqote, Male),
            [801](Miqote, Female),

            [901](Roegadyn, Male),
            [1001](Roegadyn, Female),

            [1101](Lalafell, Male),
            [1201](Lalafell, Female),

            [1301](AuRa, Male),
            [1401](AuRa, Female),

            [1501](Hrothgar, Male),
            [1601](Hrothgar, Female),

            [1701](Viera, Male),
            [1801](Viera, Female)
        }
    }
    impl GenderRace {
        pub fn split(self) -> (Gender, ModelRace) {
            match self {
                GenderRace::HyurMidlanderMale => (Gender::Male, ModelRace::Midlander),
                GenderRace::HyurMidlanderFemale => (Gender::Female, ModelRace::Midlander),
                GenderRace::HyurHighlanderMale => (Gender::Male, ModelRace::Highlander),
                GenderRace::HyurHighlanderFemale => (Gender::Female, ModelRace::Highlander),
                GenderRace::ElezenMale => (Gender::Male, ModelRace::Elezen),
                GenderRace::ElezenFemale => (Gender::Female, ModelRace::Elezen),
                GenderRace::LalafellMale => (Gender::Male, ModelRace::Lalafell),
                GenderRace::LalafellFemale => (Gender::Female, ModelRace::Lalafell),
                GenderRace::MiqoteMale => (Gender::Male, ModelRace::Miqote),
                GenderRace::MiqoteFemale => (Gender::Female, ModelRace::Miqote),
                GenderRace::RoegadynMale => (Gender::Male, ModelRace::Roegadyn),
                GenderRace::RoegadynFemale => (Gender::Female, ModelRace::Roegadyn),
                GenderRace::AuRaMale => (Gender::Male, ModelRace::AuRa),
                GenderRace::AuRaFemale => (Gender::Female, ModelRace::AuRa),
                GenderRace::HrothgarMale => (Gender::Male, ModelRace::Hrothgar),
                GenderRace::HrothgarFemale => (Gender::Female, ModelRace::Hrothgar),
                GenderRace::VieraMale => (Gender::Male, ModelRace::Viera),
                GenderRace::VieraFemale => (Gender::Female, ModelRace::Viera),
            }
        }
    }
}
