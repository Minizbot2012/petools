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
            $([$racedef:expr]($($subracedef:expr$(,)?)+)$(,)?)*
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
            #[brw(repr = u32)]
            pub enum Subrace {
                $(
                    $($subracedef,)+
                )+
            }
        }

        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
            #[repr(u8)]
            pub enum Gender {
                Male,
                Female,
            }
        }

        paste! {
            #[binrw]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, FromPrimitive)]
            #[brw(repr = u32)]
            pub enum Race {
                $(
                    $racedef,
                )+
            }
        }

        paste! {
            #[binrw]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, FromPrimitive)]
            #[brw(repr = u32)]
            pub enum GenderRace {
                $(
                    [<$race $($subrace)? $gender>] = $id,
                )+
            }
        }

        paste! {
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
        [Hyur](Midlander, Highlander),
        [Elezen](Wildwood, Duskwight),
        [Lalafell](Plainsfolk, Dunesfolk),
        [Miqote](Seeker, Keeper),
        [Roegadyn](SeaWolf, Hellsguard),
        [AuRa](Raen, Xaela),
        [Hrothgar](Hellion, Lost),
        [Viera](Rava, Veena),
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
