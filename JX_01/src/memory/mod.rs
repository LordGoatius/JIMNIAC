//! We use a page table of size 729 Words. Each word makes up 3 trytes, and therefore requires 2187
//! trytes in a page to sufficiently address a page.
//!
//! Virtual Address: ___________________________
//!                  xx\    /\    /\    /|     |
//!                      3     2     1    Offset
//! xx: Metadata Trit
//! n: n-level index
//! offset: tryte offset into page

use std::collections::HashMap;

use ternary::word::Word;

const PAGE_TABLE_SIZE: usize = 729;
const WORDS_PER_TABLE: usize = PAGE_TABLE_SIZE * 3;

type Address = Word;

struct MMU {
    tlb: [(Address, Address); PAGE_TABLE_SIZE]
}

struct Memory<'a> {
    // A reference to the CPU status register to read CPU state (MMU enable trit)
    csr: &'a Word,
    // A reference to the page table register to read lvl 3 table location
    ptr: &'a Word,
    mmu: MMU,
    memory: HashMap<Address, Page>
}

type Page = [Word; WORDS_PER_TABLE];
