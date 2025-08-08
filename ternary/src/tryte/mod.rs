use crate::trits::Trit;

/// I'm manually bit packing this because dependencies are annoying
#[repr(transparent)]
pub struct Tryte(u32);

impl From<[Trit; 9]> for Tryte {
    fn from(value: [Trit; 9]) -> Self {
        Tryte(
            value
                .into_iter()
                .enumerate()
                .map(|(i, trit)| -> u32 {
                    let val = trit as u32;
                    return val << (2 * i);
                })
                .fold(0, std::ops::BitOr::bitor),
        )
    }
}

impl From<Tryte> for [Trit; 9] {
    fn from(value: Tryte) -> Self {
        let value = value.0;
        [0, 1, 2, 3, 4, 5, 6, 7, 8]
            .map(|i| unsafe {
                std::mem::transmute::<u8, Trit>(((value >> (2 * i)) & 0b11) as u8)
            })
    }
}

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte};

    #[test]
    fn test_convert() {
        let pone = Trit::POne;
        let none = Trit::NOne;
        let zero = Trit::Zero;

        let arr = [zero, pone, pone, pone, none, none, none, zero, zero];
        let tryte: Tryte = arr.into();
        let test_eq: [Trit; 9] = tryte.into();
        assert_eq!(arr, test_eq);
    }
}
