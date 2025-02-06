use bitfield_struct::bitfield;

use crate::trits::Trit;

// use crate::{trits::*, word::Word};

//pub mod binops;
pub mod unops;
//pub mod tritops;

#[bitfield(u8)]
#[derive(PartialEq, Eq, Hash)]
pub struct Tryte {
    #[bits(2)]
    one: Trit,
    #[bits(2)]
    two: Trit,
    #[bits(2)]
    three: Trit,
    #[bits(2)]
    four: Trit,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TryteAddResult {
    pub carry: Trit,
    pub result: Tryte,
}

//=== Impl Tryte ===//

// impl Tryte {
//     fn abs(value: Self) -> Self {
//         if value < Tryte::default() {
//             -value
//         } else {
//             value
//         }
//     }
// }

// impl From<Tryte> for isize {
//     fn from(value: Tryte) -> Self {
//         value
//             .iter()
//             .enumerate()
//             .map(|(i, trit)| match trit {
//                 Trit::NOne => -isize::pow(3, i as u32),
//                 Trit::Zero => 0,
//                 Trit::POne => isize::pow(3, i as u32),
//             })
//             .sum()
//     }
// }
// 
// impl From<Tryte> for Word {
//     fn from(value: Tryte) -> Self {
//         [value, Tryte::default(), Tryte::default()].into()
//     }
// }
// 
// impl PartialOrd for Tryte {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
// 
// impl Ord for Tryte {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let self_isize:  isize = (*self).into();
// 
//         let other_isize: isize = (*other).into();
// 
//         self_isize.cmp(&other_isize)
//     }
// }

#[cfg(test)]
pub mod test {
    use crate::tryte::Tryte;

    #[test]
    fn test_size() {
        eprintln!("{}", size_of::<Tryte>());
    }
}
