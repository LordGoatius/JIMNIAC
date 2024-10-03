use std::isize;

use crate::{stack::Chunk, tryte::Tryte, word::Word};

use super::{Stack, HALF_PAGE_SIZE};

struct IndexStruct {
    index: usize,
    page_index: usize,
}

impl Stack {
    pub(super) fn insert(&mut self, addr: Word, data: Tryte) {
        let page_index_signed: isize = addr.lowest_tryte().into();
        let page_index = (page_index_signed + HALF_PAGE_SIZE as isize) as usize;
        let addr = addr.zero_lowest_tryte();
        let chunk = self.hashmap.get_mut(&addr);

        match chunk {
            None => {
                self.hashmap.insert(addr, Chunk::default());
                self.hashmap
                    .get_mut(&addr)
                    .unwrap_or_else(|| unreachable!())[page_index] = data;
            }
            Some(chunk) => {
                chunk[page_index] = data;
            }
        }
    }

    pub(super) fn get(&mut self, addr: Word) -> &Tryte {
        let page_index_signed: isize = addr.lowest_tryte().into();
        let page_index = (page_index_signed + HALF_PAGE_SIZE as isize) as usize;
        let addr = addr.zero_lowest_tryte();

        if let Some(chunk) = self.hashmap.get(&addr) {
            return &chunk[page_index];
        }

        self.hashmap
            .insert(addr, Chunk::default());

        let chunk = self
            .hashmap
            .get(&addr.zero_lowest_tryte())
            .unwrap();
        &chunk[page_index]
    }

    pub(super) fn get_mut(&mut self, addr: Word) -> &mut Tryte {
        let page_index_signed: isize = addr.lowest_tryte().into();
        let page_index = (page_index_signed + HALF_PAGE_SIZE as isize) as usize;
        let addr = addr.zero_lowest_tryte();

        if let Some(chunk) = self.hashmap.get_mut(&addr) {
            return &mut chunk[page_index];
        }

        self.hashmap
            .insert(addr, Chunk::default());

        let chunk = self
            .hashmap
            .get_mut(&addr.zero_lowest_tryte())
            .unwrap();
        &mut chunk[page_index]
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        stack::{Stack, HALF_PAGE_SIZE, PAGE_SIZE},
        trits::Trit,
        tryte::Tryte, word::Word,
    };

    #[test]
    fn test_stack() {
        let mut stack = Stack::default();

        let min_tryte: Tryte = [Trit::NOne; 9].into();
        let zer_tryte: Tryte = [Trit::Zero; 9].into();
        let max_tryte: Tryte = [Trit::POne; 9].into();

        let addr: Word = [max_tryte, zer_tryte, zer_tryte].into();
        stack.insert(addr, max_tryte);
        assert_eq!(*stack.get(addr), max_tryte);

        let addr = (addr + Trit::POne).result;
        stack.insert(addr, min_tryte);
        assert_eq!(*stack.get(addr), min_tryte);
        println!("{stack:#?}");

    }

    #[test]
    fn test_isize_min() {
        let min_tryte: Tryte = [Trit::NOne; 9].into();
        let max_tryte: Tryte = [Trit::POne; 9].into();
        let test0: isize = min_tryte.into();
        let test1: isize = max_tryte.into();

        assert_eq!(test0, -(HALF_PAGE_SIZE as isize));
        assert_eq!(test0 + HALF_PAGE_SIZE as isize, 0);
        assert_eq!(test1 + HALF_PAGE_SIZE as isize, PAGE_SIZE as isize - 1);
    }
}
