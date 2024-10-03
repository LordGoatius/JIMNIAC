use std::{collections::HashMap, ops::{Deref, DerefMut}, usize};

use crate::{tryte::Tryte, word::Word};

pub mod hash;
pub mod errors;

const PAGE_SIZE: usize      = 3usize.pow(9); // 19683
const HALF_PAGE_SIZE: usize = PAGE_SIZE / 2;  // 9841

#[derive(Debug)]
pub struct Stack {
    hashmap: HashMap<Word, Chunk>
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            hashmap: HashMap::default()
        }
    }
}


#[derive(Debug, Clone)]
struct Chunk([Tryte; PAGE_SIZE]);

impl Deref for Chunk {
    type Target = [Tryte; PAGE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk([Tryte::default(); PAGE_SIZE])
    }
}

#[cfg(test)]
pub mod test {
    use crate::{trits::Trit, tryte::Tryte, word::Word};


    use super::{Stack, PAGE_SIZE};

    #[test]
    fn tryte_usize_conversion() {
        let mut tryte: Tryte = [Trit::NOne; 9].into();
        let mut isize: isize = -19683 / 2;

        while tryte != [Trit::POne; 9].into() {
            let tryte_as_isize: isize = tryte.into();
            assert_eq!(isize, tryte_as_isize);
            if tryte == [Trit::NOne; 9].into() {
            }
            isize += 1;
            tryte = (tryte + Trit::POne).result;
        }
        let tryte_as_isize: isize = tryte.into();
        assert_eq!(isize, tryte_as_isize);
    }

    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        let mut stack = Stack::default();

        let min_tryte: Tryte = [Trit::NOne; 9].into();
        let zer_tryte: Tryte = [Trit::Zero; 9].into();
        let max_tryte: Tryte = [Trit::POne; 9].into();

        let nzp = [min_tryte, zer_tryte, max_tryte];

        let mut addr: Word = [zer_tryte, zer_tryte, zer_tryte].into();

        b.iter(|| {
            for i in 0..(2*PAGE_SIZE) {
                stack.insert(addr, nzp[i % 3]);
                addr = (addr + Trit::POne).result;
            }
            addr = [zer_tryte; 3].into();
            for i in 0..(2*PAGE_SIZE) {
                let val = stack.get(addr);
                addr = (addr + Trit::POne).result;
                assert_eq!(*val, nzp[i % 3]);
            }
        })
    }
}
