use JX_01::isa::registers::Register;
use ternary::trits::Trit;

pub enum Item<'src> {
    Orig(isize),
    Label(&'src str),
    Data(Data<'src>),
    Instr(Instr<'src>),
    Line(Line),
}

pub enum Size {
    Tryte,
    Word,
}

pub enum Value<'src> {
    Num(isize),
    Label(&'src str),
}

pub struct Data<'src> {
    times: Option<u32>,
    size: Size,
    value: Value<'src>
}

pub struct Line {
    coord1: (isize, isize),
    coord2: (isize, isize),
    color: [Trit; 3]
}

// This one includes pseudoinstructions like mov
pub enum Instr<'src> {
    IOp {
        op: Op,
        r1: Register,
        imm: Option<Value<'src>>
    },
    ROp {
        op: Op,
        r1: Register,
        r2: Register,
        imm: Option<Value<'src>>
    },
    HALT,
    DTI,
    STI,
    WFI,
    RTI,
    LIT {
        r1: Register,
    },
    INTERRUPT {
        imm: isize,
    },
    EGPU {
        r1: Register,
    },
    LVB {
        r1: Register, imm: isize,
    },
    EGEL {
        r1: Register,
    },
    PCSR,
    PPSR,
    PPTR,
    POCSR,
    POPSR,
    POPTR,
    LPT {
        r1: Register,
    },
    INTM {
        imm: isize,
    },
    INTE {
        imm: isize,
    },
    INTS {
        imm: isize,
    },
    IN {
        r1: Register,
        imm: isize,
    },
    OUT {
        r1: Register,
        imm: isize,
    },
    CALL {
        r1: Register,
        imm: isize,
    },
    RET,
}

pub enum Op {
    BPN,
    BPP,
    BPZ,
    BGQ,
    BLQ,
    BLT,
    BGT,
    BNE,
    BEQ,
    JMP,
    CMP,
    STRE,
    LOAD,
    ADD,
    SUB,
    MUL,
    QOT,
    REM,
    AND,
    OR,
    SFT,
    NOT,
    ROT,
    PUSH,
    POP,
    MOV,
}
