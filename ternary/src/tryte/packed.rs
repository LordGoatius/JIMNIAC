use std::{array, ops::{Deref, DerefMut}};
use packed_struct::{prelude::*, types::bits::ByteArray};

use crate::trits::Trit;

use super::Tryte;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PackedTryte([u8; 3]);

impl Deref for PackedTryte {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PackedTryte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ByteArray for PackedTryte {
    fn len() -> usize {
        3
    }

    fn as_bytes_slice(&self) -> &[u8] {
        &self[..]
    }

    fn as_mut_bytes_slice(&mut self) -> &mut [u8] {
        &mut self[..]
    }

    fn new(value: u8) -> Self {
        PackedTryte([value; 3])
    }

    fn rotate_right(&mut self, bytes: usize) {
        let arr_mut = &mut self.0;
        arr_mut.rotate_right(bytes);
    }
}

impl PackedStruct for Tryte {
    type ByteArray = [u8; 3];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let packed: u32 = self.iter().enumerate().fold(0, |acc, (i, &val)| {
            acc | ((val as u32) << (2 * (8 - i)))        
        });

        Ok([
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            (packed & 0xFF) as u8,
        ])

    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        let bits = ((src[0] as u32) << 16) | ((src[1] as u32) << 8) | (src[2] as u32);

        let arr: [Trit; 9] = array::from_fn(|i| {
            let val = ((bits >> (2 * (8 - i))) & 0b11) as u8;
            Trit::from_primitive(val).unwrap()
        });

        Ok(Tryte(arr))
    }
}

#[cfg(test)]
pub mod test {
    use packed_struct::PackedStruct;

    use super::{Trit, Tryte};

    #[test]
    fn test_packing() {
        let n_one = Trit::NOne;
        let zero = Trit::Zero;
        let p_one = Trit::POne;

        let mut start = Tryte([n_one; 9]);
        let one = Tryte([p_one, zero, zero, zero, zero, zero, zero, zero, zero]);

        // Fun fact this enumerates the bijection between + to - $\pm\frac{3^9 - 1}{2}$ $\in$
        // $\mathbb{Z}$
        while start != one {
            let packed = start.pack().unwrap();
            eprintln!("{packed:?}");
            assert_eq!(start, Tryte::unpack(&packed).unwrap());
            start = (start + p_one).result;
        }
    }

}
