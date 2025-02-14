pub mod septivingtimal {
    use ternary::trits::Trit::{self, *};

    const Z:    [Trit; 3] = [NOne, NOne, NOne];
    const Y:    [Trit; 3] = [Zero, NOne, NOne];
    const X:    [Trit; 3] = [POne, NOne, NOne];
    const W:    [Trit; 3] = [NOne, Zero, NOne];
    const V:    [Trit; 3] = [Zero, Zero, NOne];
    const U:    [Trit; 3] = [POne, Zero, NOne];
    const T:    [Trit; 3] = [NOne, POne, NOne];
    const S:    [Trit; 3] = [Zero, POne, NOne];
    const R:    [Trit; 3] = [POne, POne, NOne];
    const Q:    [Trit; 3] = [NOne, NOne, Zero];
    const P:    [Trit; 3] = [Zero, NOne, Zero];
    const O:    [Trit; 3] = [POne, NOne, Zero];
    const N:    [Trit; 3] = [NOne, Zero, Zero];
    const ZERO: [Trit; 3] = [Zero, Zero, Zero];
    const A:    [Trit; 3] = [POne, Zero, Zero];
    const B:    [Trit; 3] = [NOne, POne, Zero];
    const C:    [Trit; 3] = [Zero, POne, Zero];
    const D:    [Trit; 3] = [POne, POne, Zero];
    const E:    [Trit; 3] = [NOne, NOne, POne];
    const F:    [Trit; 3] = [Zero, NOne, POne];
    const G:    [Trit; 3] = [POne, NOne, POne];
    const H:    [Trit; 3] = [NOne, Zero, POne];
    const I:    [Trit; 3] = [Zero, Zero, POne];
    const J:    [Trit; 3] = [POne, Zero, POne];
    const K:    [Trit; 3] = [NOne, POne, POne];
    const L:    [Trit; 3] = [Zero, POne, POne];
    const M:    [Trit; 3] = [POne, POne, POne];
}
