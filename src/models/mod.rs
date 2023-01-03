pub mod gmp;
pub mod eqp;
pub mod eqdp;
pub mod est;
pub mod imc;
use serde::{Deserialize, Serialize};

use self::eqdp::EqdpJson;
use self::eqp::EqpJson;
use self::gmp::GmpJson;
use self::est::EstJson;
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[serde(tag = "Type", content = "Manipulation")]
pub enum MetaManipulation {
    Eqdp(EqdpJson),
    Eqp(EqpJson),
    Gmp(GmpJson),
    Est(EstJson),
}
