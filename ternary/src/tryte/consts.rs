use crate::trits::Trit;

use super::Tryte;

pub const TRYTE_MIN: Tryte = Tryte([Trit::NOne; 9]);
pub const TRYTE_MAX: Tryte = Tryte([Trit::POne; 9]);

