use std::array;

use JX_01::{gpu::make_line, isa::{ADD_T, ALU_CTRL_R_RI, ALU_CTRL_R_RR, BEQ_T, CALL_CTRL_R, CMP_T, Instr, code::DecEncExt, encode, registers::*}};
use ternary::{trits::Trit, word::Word};

fn main() {
        use Instr::*;

        // main:
        // 00  NOP
        // 03  mov %r1, 0       ; r1 is for indexing into array
        // 06  mov %r2, gpu     ; r2 is for storing GPU location
        // 09  egpu %r2
        // 12  mov %r3, vec_buf ; r3 never changes 
        // 15  mov %r4, idt     ; r4 idt location
        // 18  lidt %r4
        // loop:
        // 21  cmp %r0, %r0
        // 24  beq loop
        // int1:
        // 27  call array, %r1
        // 30  add %r1, 6
        // 33  rti
        // array:
        // 36  lvb %r3, word 03 ; h (3)
        // 39  ret
        // 42  lvb %r3, word 07 ; e (4)
        // 45  ret
        // 48  lvb %r3, word 09 ; l (2)
        // 51  ret
        // 54  lvb %r3, word 11 ; l (2)
        // 56  ret
        // 60  lvb %r3, word 15 ; o (4)
        // 63  ret
        // 66  lvb %r3, word 19 ; w (4)
        // 69  ret
        // 72  lvb %r3, word 23 ; o (4)
        // 75  ret
        // 78  lvb %r3, word 27 ; r (4)
        // 81  ret
        // 84  lvb %r3, word 29 ; l (2)
        // 87  ret
        // 90  lvb %r3, word 32 ; d (3)
        // 93  ret
        // gpu:
        // 96  word vec_buf
        // 99  word 0
        // idt:
        // 102 word 0
        // 105 word int1
        // vec_buf:
        // 108 warword 'h' 
        //     warword 'e' 
        //     warword 'l' 
        //     warword 'l' 
        //     warword 'o' 
        //     warword 'w' 
        //     warword 'o' 
        //     warword 'r' 
        //     warword 'l' 
        //     warword 'd' 
        const IDT_LOC: isize = 102;
        const GPU_LOC: isize = 96;
        const VEC_BUF: isize = 108;

        const X: [isize; 64] = [
            // Hello
            -200,-200,-200,-100,-100,-100,
            -70,-70,-70,0,-70,-20,-70,0,
            30,30,30,81,
            105,105,105,156,
            181,181,181,240,240,240,240,181,
            // World
             -235,-205,-205,-175,-175,-145,-145,-115,
             -75,-75,-75,5,5,5,5,-75,
             35,35,35,85,85,35,35,85,
             115,115,115,175,
             215,215,215,275,275,215,
        ];
        const Y: [isize; 64] = [
            // Hello
            200,50,125,125,200,50,
            200,50,200,200,125,125,50,50,
            200,50,50,50,
            200,50,50,50,
            200,50,50,50,50,200,200,200,
            // World
            -50,-200,-200,-50,-50,-200,-200,-50,
            -50,-200,-200,-200,-200,-50,-50,-50,
            -50,-200,-50,-90,-90,-130,-130,-200,
            -50,-200,-200,-200,
            -50,-200,-50,-125,-125,-200,
        ];

        let instrs = [
            OPRR(ALU_CTRL_R_RR, ADD_T, N0, N0, Word::ZERO),
            // main:
            OPRI(ALU_CTRL_R_RI, ADD_T, N1, Word::ZERO),     // 03  mov %r1, 0       ; r1 is for indexing into array
            OPRI(ALU_CTRL_R_RI, ADD_T, N2, GPU_LOC.into()), // 06  mov %r2, gpu     ; r2 is for storing GPU location
            EGPU(N2), // 09  egpu %r2
            OPRI(ALU_CTRL_R_RI, ADD_T, N3, VEC_BUF.into()), // 12  mov %r3, vec_buf ; r3 never changes 
            OPRI(ALU_CTRL_R_RI, ADD_T, N4, IDT_LOC.into()), // 15  mov %r4, idt     ; r4 idt location
            LIT(N4), // 18  lidt %r4
            // loop:
            OPRR(ALU_CTRL_R_RR, CMP_T, N0, N0, Word::ZERO), // 21  cmp %r0, %r0
            OPRI(ALU_CTRL_R_RI, BEQ_T, N0, 21.into()), // 24  beq loop
            // int1:
            CALL(N1, CALL_CTRL_R, 36.into()), // 27  call array, %r1
            OPRI(ALU_CTRL_R_RI, ADD_T, N1, 6.into()), // 30  add %r1, 6
            RTI, // 33  rti
            // array:
            LVB(N3, 03.into()), // 36  lvb %r3, word 03 ; h (3)
            RET,                // 39  ret
            LVB(N3, 07.into()), // 42  lvb %r3, word 07 ; e (4)
            RET,                // 45  ret
            LVB(N3, 09.into()), // 48  lvb %r3, word 09 ; l (2)
            RET,                // 51  ret
            LVB(N3, 11.into()), // 54  lvb %r3, word 11 ; l (2)
            RET,                // 56  ret
            LVB(N3, 15.into()), // 60  lvb %r3, word 15 ; o (4)
            RET,                // 63  ret
            LVB(N3, 19.into()), // 66  lvb %r3, word 19 ; w (4)
            RET,                // 69  ret
            LVB(N3, 23.into()), // 72  lvb %r3, word 23 ; o (4)
            RET,                // 75  ret
            LVB(N3, 27.into()), // 78  lvb %r3, word 28 ; r (4)
            RET,                // 81  ret
            LVB(N3, 29.into()), // 84  lvb %r3, word 30 ; l (2)
            RET,                // 87  ret
            LVB(N3, 32.into()), // 90  lvb %r3, word 33 ; d (3)
            RET,                // 93  ret
        ];

        // gpu:
        // 96  word vec_buf
        // 99  word 0
        let mut gpu: [Word; 2] = [VEC_BUF.into(), Word::ZERO];

        // idt:
        // 102 word 0
        // 105 word int1
        let mut idt: Vec<Word> = vec![
            Word::ZERO,
            27.into()
        ];

        // 10 colors
        // red
        // orange
        // yellow
        // yellow green
        // green
        // blue green
        // blue
        // indigo
        // purple
        // white
        let colors: [[Trit; 3]; 10] = {
            use Trit::*;
            [
                [POne, NOne, NOne],
                [POne, Zero, Zero],
                [POne, POne, NOne],
                [POne, POne, Zero],
                [NOne, POne, NOne],
                [NOne, POne, Zero],
                [NOne, NOne, POne],
                [Zero, NOne, POne],
                [POne, NOne, POne],
                [POne, POne, POne],
            ]
        };

        let color_index: [usize; 32] = [0,0,0,1,1,1,1,2,2,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,9,9,9];

        // vec_buf:
        // 108 warword 'h' 
        //     warword 'e' 
        //     warword 'l' 
        //     warword 'l' 
        //     warword 'o' 
        //     warword 'w' 
        //     warword 'o' 
        //     warword 'r' 
        //     warword 'l' 
        //     warword 'd'
        let mut vec_buf: [Word; 32] = array::from_fn(|i| {
            make_line((X[2*i], Y[2*i]), (X[(2*i)+1],Y[(2*i)+1]), colors[color_index[i]])
        });

        instrs.check();

        let mut data: Vec<Word> = instrs.into_iter().map(encode).collect();
        data.extend_from_slice(&mut gpu);
        data.extend_from_slice(&mut idt);
        data.extend_from_slice(&mut vec_buf);

        let mut cpu = JX_01::cpu::JX_01::new();
        cpu.import_memory(&data);
        cpu.run_program();
} 
