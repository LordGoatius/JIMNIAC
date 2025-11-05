//! We use a page table of size 729 Words. Each word makes up 3 trytes, and therefore requires 2187
//! trytes in a page to sufficiently address a page.
//!
//! Virtual Address: ___________________________
//!                  xx\    /\    /\    /|     |
//!                      3     2     1    Offset
//! xx: Metadata Trits
//! n: n-level index
//! offset: tryte offset into page

use std::collections::HashMap;

use ternary::word::Word;

/// A page table contains 729 ternary words, or 2187 trytes
pub const PAGE_TABLE_SIZE: usize = 729;

pub type Address = Word;

pub struct MMU {
    tlb: [(Address, Option<Address>); PAGE_TABLE_SIZE],
}

pub struct Memory<'a> {
    // A reference to the CPU status register to read CPU state (MMU enable trit)
    csr: &'a Word,
    // A reference to the page table register to read lvl 3 table location
    ptr: &'a Word,
    mmu: MMU,
    memory: HashMap<u64, Page>,
}

pub type Page = [Word; PAGE_TABLE_SIZE];
pub const EMPTY_PAGE: Page = [Word::ZERO; PAGE_TABLE_SIZE];

impl MMU {
    pub fn clear(&mut self) {
        // don't clear the key, but clear the value.
        // Is this a security concern absolutely waiting to occur?
        // Yes.
        self.tlb
            .iter_mut()
            .map(|addr| *addr = (addr.0, None))
            .collect()
    }
}

impl<'a> Memory<'a> {
    pub(crate) fn get_physical_word(&mut self, index: Address) -> &Word {
        // We want the first 7 trits to be zero, but the next 20 to be our type
        let addr_page = ((index.num() & (0b1111111111111111111111111111111111111111 << 14))
            | (Word::ZERO.num() & 0b11111111111111))
            & Word::WORD_BIT_MASK;
        let addr_elem = unsafe {
            Word::from_u64(
                ((index.num() & 0b11111111111111) | (Word::ZERO.num() << 14)) & Word::WORD_BIT_MASK,
            )
        };

        let page = if self.memory.contains_key(&addr_page) {
            self.memory.get(&addr_page).unwrap()
        } else {
            let _ = self.memory.insert(addr_page, EMPTY_PAGE);
            self.memory.get(&addr_page).unwrap()
        };
        
        // let page = match self.memory.get(&addr_page) {
        //     Some(page) => page,
        //     None => {
        //         let _ = self.memory.insert(addr_page, EMPTY_PAGE);
        //         self.memory.get(&addr_page).unwrap()
        //     }
        // };

        let index: isize = addr_elem.into();
        let index = index + (729 / 2);
        &page[index as usize]
    }

    pub(crate) fn get_physical_word_mut(&mut self, index: Address) -> &mut Word {
        // We want the first 7 trits to be zero, but the next 20 to be our type
        let addr_page = ((index.num() & (0b1111111111111111111111111111111111111111 << 14))
            | (Word::ZERO.num() & 0b11111111111111))
            & Word::WORD_BIT_MASK;
        let addr_elem = unsafe {
            Word::from_u64(
                ((index.num() & 0b11111111111111) | (Word::ZERO.num() << 14)) & Word::WORD_BIT_MASK,
            )
        };

        let page = if self.memory.contains_key(&addr_page) {
            self.memory.get_mut(&addr_page).unwrap()
        } else {
            let _ = self.memory.insert(addr_page, EMPTY_PAGE);
            self.memory.get_mut(&addr_page).unwrap()
        };
        
        // let page = match self.memory.get_mut(&addr_page) {
        //     Some(page) => page,
        //     None => {
        //         let _ = self.memory.insert(addr_page, EMPTY_PAGE);
        //         self.memory.get_mut(&addr_page).unwrap()
        //     }
        // };

        let index: isize = addr_elem.into();
        let index = index + (729 / 2);
        &mut page[index as usize]
    }
}
