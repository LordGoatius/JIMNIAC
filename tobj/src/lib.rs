//! a.out inspired ternary object file

use ternary::word::Word;
type Addr = isize;

#[repr(C)]
pub struct TObj<'data> {
    header: TObjHeader,
    data: &'data [Word]
}

#[repr(C)]
pub struct TObjHeader {
    magic:  u64,
    text:   u64,
    data:   u64,
    bss:    u64,
    sym:    u64,
    entry:  u64,
    textr:  u64,
    datar:  u64,
}
