use std::{array, ops::{Deref, DerefMut}};
use packed_struct::{prelude::*, types::bits::ByteArray};

use crate::trits::Trit;

use super::Word;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PackedWord([u8; 7]);

impl Deref for PackedWord {
    type Target = [u8; 7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PackedWord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ByteArray for PackedWord {
    fn len() -> usize {
        7
    }

    fn as_bytes_slice(&self) -> &[u8] {
        &self[..]
    }

    fn as_mut_bytes_slice(&mut self) -> &mut [u8] {
        &mut self[..]
    }

    fn new(value: u8) -> Self {
        PackedWord([value; 7])
    }

    fn rotate_right(&mut self, bytes: usize) {
        let arr_mut = &mut self.0;
        arr_mut.rotate_right(bytes);
    }
}

impl PackedStruct for Word {
    type ByteArray = [u8; 7];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let packed: u64 = self.iter().enumerate().fold(0, |acc, (i, &val)| {
            acc | ((val as u64) << (2 * (26 - i)))        
        });

        Ok([
            ((packed >> 48) & 0xFF) as u8,
            ((packed >> 40) & 0xFF) as u8,
            ((packed >> 32) & 0xFF) as u8,
            ((packed >> 24) & 0xFF) as u8,
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            (packed & 0xFF) as u8,
        ])

    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let bits =
            (src[0] as u64) << 48 |
            (src[1] as u64) << 40 |
            (src[2] as u64) << 32 |
            (src[3] as u64) << 24 |
            (src[4] as u64) << 16 |
            (src[5] as u64) << 8 |
            (src[6] as u64);

        let arr: [Trit; 27] = array::from_fn(|i| {
            let val = ((bits >> (2 * (26 - i))) & 0b11) as u8;
            Trit::from_primitive(val).unwrap()
        });

        Ok(Word(arr))
    }
}

#[cfg(test)]
pub mod test {
    use packed_struct::PackedStruct;

    use super::{Trit, Word};

    #[test]
    fn test_packing() {
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let mut start = Word([zero; 27]);

        // Fun fact this enumerates the bijection between + to - $\pm\frac{3^9 - 1}{2}$ $\in$
        // $\mathbb{Z}$
        for _ in 0..0x100000 {
            let packed = start.pack().unwrap();
            assert_eq!(start, Word::unpack(&packed).unwrap());
            start = (start + p_one).result;
        }
    }

}
