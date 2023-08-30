use binrw::binrw;
use num_derive::FromPrimitive;
use paste::paste;
use serde::{Deserialize, Serialize};
use strum::EnumString;

macro_rules! gender_race_conv {
    ($race:expr, $subrace:expr, $gender:expr) => {
        paste! {
            (Gender::$gender, ModelRace::$subrace)
        }
    };
    ($race:expr, $gender:expr) => {
        paste! {
            (Gender::$gender, ModelRace::$race)
        }
    };
}

macro_rules! define_races {
    (
	    Race {
            $([$id:expr]($race:expr, $gender:expr $(, $subrace:expr)?),)*
        }
        RaceSubrace {
            $([$racedef:expr]($($subracedef:expr, $subraceid:expr$(,)?)+)$(,)?)*
        }
        Models {
            $($models:expr,)*
        }
    ) => {
        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
            #[strum(ascii_case_insensitive)]
            pub enum ModelRace {
                Unknown,
                $($models,)*
            }
        }

        paste! {
            #[binrw]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
            #[brw(repr(u8), little)]
            pub enum Subrace {
                Unknown = 0,
                $(
                    $($subracedef,)+
                )+
            }
            #[allow(dead_code)]
            impl Subrace {
                pub fn to_id(self) -> u8 {
                    match self {
                        $(
                            $(Subrace::[<$subracedef>] => $subraceid,)+
                        )+
                        Subrace::Unknown => 0,
                    }
                }
                pub fn from_id(id: u8) -> Subrace {
                    match id {
                        $(
                            $($subraceid => Subrace::[<$subracedef>],)+
                        )+
                        _ => Subrace::Unknown,
                    }
                }
            }
        }

        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
            #[binrw]
            #[brw(repr(u8),little)]
            pub enum Gender {
                Male = 0,
                Female,
            }
        }

        paste! {
            #[binrw]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
            #[brw(repr(u32),little)]
            pub enum Race {
                Unknown,
                $(
                    $racedef,
                )+
            }
        }

        paste! {
            #[binrw]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, FromPrimitive)]
            #[brw(repr(u32),little)]
            pub enum GenderRace {
                $(
                    [<$race $($subrace)? $gender>] = $id,
                )+
            }

            impl GenderRace {
                pub fn split(self) -> (Gender, ModelRace) {
                    match self {
                        $(
                            GenderRace::[<$race $($subrace)? $gender>] => gender_race_conv!($race, $($subrace,)? $gender),
                        )+
                    }
                }

                #[allow(dead_code)]
                pub fn gender(self) -> Gender {
                    match self {
                        $(
                            GenderRace::[<$race $($subrace)? $gender>] => Gender::$gender,
                        )+
                    }
                }
                #[allow(dead_code)]
                pub fn race(self) -> Race {
                    match self {
                        $(
                            GenderRace::[<$race $($subrace)? $gender>] => Race::$race,
                        )+
                    }
                }
                #[allow(dead_code)]
                pub fn id(self) -> u32 {
                    match self {
                        $(
                            GenderRace::[<$race $($subrace)? $gender>] => $id,
                        )+
                    }
                }
            }
        }
    };
}

define_races! {
    Race {
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
        [1801](Viera, Female),
    }
    RaceSubrace {
        [Hyur](Midlander, 1, Highlander, 2),
        [Elezen](Wildwood, 3, Duskwight, 4),
        [Lalafell](Plainsfolk, 5, Dunesfolk, 6),
        [Miqote](Seeker, 7, Keeper, 8),
        [Roegadyn](SeaWolf, 9, Hellsguard, 10),
        [AuRa](Raen, 11, Xaela, 12),
        [Hrothgar](Hellion, 13, Lost, 14),
        [Viera](Rava, 15, Veena, 16),
    }
    Models {
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
}
