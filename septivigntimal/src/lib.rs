use ternary::trits::Trit::{self, *};

pub type Tribble = [Trit; 3];

pub const Z:    Tribble = [NOne, NOne, NOne];
pub const Y:    Tribble = [Zero, NOne, NOne];
pub const X:    Tribble = [POne, NOne, NOne];
pub const W:    Tribble = [NOne, Zero, NOne];
pub const V:    Tribble = [Zero, Zero, NOne];
pub const U:    Tribble = [POne, Zero, NOne];
pub const T:    Tribble = [NOne, POne, NOne];
pub const S:    Tribble = [Zero, POne, NOne];
pub const R:    Tribble = [POne, POne, NOne];
pub const Q:    Tribble = [NOne, NOne, Zero];
pub const P:    Tribble = [Zero, NOne, Zero];
pub const O:    Tribble = [POne, NOne, Zero];
pub const N:    Tribble = [NOne, Zero, Zero];
pub const ZERO: Tribble = [Zero, Zero, Zero];
pub const A:    Tribble = [POne, Zero, Zero];
pub const B:    Tribble = [NOne, POne, Zero];
pub const C:    Tribble = [Zero, POne, Zero];
pub const D:    Tribble = [POne, POne, Zero];
pub const E:    Tribble = [NOne, NOne, POne];
pub const F:    Tribble = [Zero, NOne, POne];
pub const G:    Tribble = [POne, NOne, POne];
pub const H:    Tribble = [NOne, Zero, POne];
pub const I:    Tribble = [Zero, Zero, POne];
pub const J:    Tribble = [POne, Zero, POne];
pub const K:    Tribble = [NOne, POne, POne];
pub const L:    Tribble = [Zero, POne, POne];
pub const M:    Tribble = [POne, POne, POne];

#[cfg(test)]
pub mod tests {
    use ternary::tryte::Tryte;

    use crate::*;

    #[test]
    fn const_eval() {
        let arr: Tryte = [ZERO, ZERO, ZERO].into();
        assert_eq!(arr, Tryte::ZERO);
        let arr: Tryte = [A, ZERO, ZERO].into();
        assert_eq!(arr, Tryte::PONE);
        let arr: Tryte = [N, ZERO, ZERO].into();
        assert_eq!(arr, Tryte::NONE);
    }
}
