---
title: "JX_01: Computers Don't Need to be Binary"
author: Jimmy Ostler
options:
  implicit_slide_ends: true
theme:
  name: dark
---

Sections
===
<!-- font_size: 2 -->
<!-- alignment: center -->
- Motivation
- Technical Background
  - Ternary as a number system
  - Ternary in the context of computing
- Development Process
  - Github
  - Actions, Testing, and Releasing
- Next Steps
- Demo

Motivation
---
<!-- font_size: 2 -->

Most people have heard of:
- binary computers
- *maybe* analog computers

<!-- newline -->

There is no reason they have to be binary. In fact, there *historically* exist
ternary computers, called the **Setun**, from the USSR.

I want to emulate a ternary computer, identify if there are any advantages over binary,
and learn about the foundations of computers from the beginning.

I want to emulate a computer *fundamentally different* from any computer that currently exists.

Requirements
---
<!-- font_size: 2 -->

It must:
- Be turing complete
- Be capable of I/O
- Have Graphics Capabilities

In other words, apollo engineers should be able to go to the moon with this.

Technical Background
---
<!-- font_size: 2 -->
Computers, in essence, speak numbers.

"Binary code" or machine code, is simply a special number that can be decoded into
instructions that computers understand.

To do this, we need ternary numbers

Balanced Ternary
---
<!-- font_size: 2 -->
A ternary system uses 3 digits. Mine uses, 0, 1, and -1 (we use T to represent -1).

Just like a normal alternative base system, each digit represents 3^n, where n is the
digit's position.

```
                        ┌──┬─┬─┬─┬─┐              
                    num:│1 │T│0│T│1│              
                        └──┴─┴─┴─┴─┘              
    digit position (n):  4  3 2 1 0               
value multiplier (3^n): 81 27 9 3 1               
                                                  
    (1*81)+(T*27)+(0*9)+(T*3)+(1*1) = 52 (decimal)
```

Here is a ternary number, and how we can find it's value.

<!-- font_size: 2 -->
Ternary has some nice properties!
- native negative numbers
- sign is always the highest nonzero digit

My Computer
---
<!-- font_size: 2 -->
Computers work on numbers of fixed size, usually a power of 2.
- Mine works on ternary numbers of size 27.

We can turn a 27 digit number into a command for the computer easily:
```
   ┌───────────────────────────┐              
   │nnnnnnnnnnnnnnnnnnnnnnnnnnn├───┐          
   └───────────────────────────┘   │Split into
  ┌─────────┬─────────┬─────────┐  │three     
  │aaaaaaaaa│bbbbbbbbb│ccccccccc│◄─┘          
┌─┴─────────┼─────────┼─────────┴┐            
│  9 digits │9 for the│9 for the │            
│  for the  │first arg│second arg│            
│instruction│         │          │            
└───────────┴─────────┴──────────┘            
```

(Aside)
---
<!-- font_size: 2 -->
I invented a ternary hexadecimal equivalent, called septivigntimal

```
There exists a need for an alternative to hexadecimal.
Several people have tried, none succeeded to my
satisfaction. I propose the following:

  TTT TT0 TT1 T0T T00 T01 T1T T10 T11 0TT 0T0 0T1 00T
  -13 -12 -11 -10  -9  -8  -7  -6  -5  -4  -3  -2  -1     
    Z   Y   X   W   V   U   T   S   R   Q   P   O   N
    
  111 110 11T 101 100 10T 1T1 1T0 1TT 011 010 01T 001
   13  12  11  10   9   8   7   6   5   4   3   2   1     
    M   L   K   J   I   H   G   F   E   D   C   B   A     
                                                 
 And 0 remains 0. Notably, the only number is 0. 
```
This allows representing a 27 digit ternary number as
a 9 digit septivigntimal number.

Instructions
---
<!-- font_size: 2 -->
I can then use a single septivigntimal number to represent my instruction,
and use the rest for arguments

```
 ALU:  │ STACK: │ BRANCH:
───────┼────────┼────────
 add A │ push K │ cmp  Q 
 sub B │ pop  L │ beq  R 
 mul C │ call M │ bne  S 
 qot D │ ret  N │ bgt  T 
 rem E │ entr M │ blt  U 
 and F │ leve N │ blq  V 
 or  G ├────────┤ bgq  W 
       │ MEM:   │        
 sft H ├────────┤ bpz  X 
 not I │ load O │ bpp  Y 
 rot J │ str  P │ bpn  Z 
```

Managing this Process
---
<!-- font_size: 2 -->
This is a complicated process, with lots to manage.

I needed to manage these things:
- Hosting
- Programming and Building
- Testing

To host and test, I used Github and Github Actions. This allowed me to test my code
every time I pushed upstream.

For programming and building, I used Cargo, the build system for Rust, the language I used.

Cargo's built in commands made using Github actions for deployment and testing extremely easy.

Testing
---
<!-- font_size: 2 -->
Rust allows you to annotate functions with `#[test]`, and automatically
run all of them with the command `cargo test`.

They can be simple:
<!-- font_size: 1 -->
```rust
fn test_ord() {
    let min: Tryte = [NONE; 9].into();
    let val1: isize = min.into();
    let mut min: Word = min.into();
    let val2: isize = min.into();
    assert_eq!(val1, val2);
    for _ in 0..9841 {
        let add = min + PONE;
        assert!(add > min, "{add:?}, {min:?}");
        min = add;
    }
}
```

Testing
---
...or complex
```rust
let n: Word = 6.into();

/* absurdly long assembly source code comment omitted */
let instrs = [
    /* 00 */ OPRI(ALU_CTRL_R_RI, ADD_T, NN11, 2.into()),
    /* 03 */ OPRI(ALU_CTRL_R_RI, ADD_T, NN13, n.into()),
    /* 06 */ CALL(N0, CALL_CTRL_R, 15.into()),
    /* 09 */ OPRR(ALU_CTRL_R_RR, CMP_T, N0, N0, Word::ZERO),
    /* 12 */ OPRI(ALU_CTRL_R_RI, BEQ_T, N0, 48.into()),
    /* 15 */ OPRR(ALU_CTRL_R_RR, CMP_T, NN13, NN11, Word::ZERO),
    /* 18 */ OPRI(ALU_CTRL_R_RI, BGT_T, N0, 30.into()),
    /* 21 */ OPRI(ALU_CTRL_R_RI, ADD_T, NN12, 2.into()),
    /* 24 */ OPRR(ALU_CTRL_R_RR, CMP_T, N0, N0, Word::ZERO),
    /* 27 */ OPRI(ALU_CTRL_R_RI, BEQ_T, N0, 45.into()),
    /* 30 */ OPRI(ALU_CTRL_R_RI, PUSH_T, NN13, Word::ZERO),
    /* 33 */ OPRI(ALU_CTRL_R_RI, ADD_T, NN13, Word::NONE),
    /* 36 */ CALL(N0, CALL_CTRL_R, 15.into()),
    /* 39 */ OPRI(ALU_CTRL_R_RI, POP_T, NN10, Word::ZERO),
    /* 42 */ OPRR(ALU_CTRL_R_RR, MUL_T, NN12, NN10, Word::ZERO),
    /* 45 */ RET,
    /* 48 */ HALT,
];

instrs.check();

let mut cpu = JX_01::new();
cpu.import_instrs(&instrs);
cpu.run_program();

let fact = |mut n| {
    let none = Word::NONE;
    let mut prod = Word::PONE;
    while n > Word::ZERO {
        prod = prod * n;
        n = n + none;
    }
    prod
};

assert_eq!(cpu.registers.get_word(NN12), fact(n));
```

Deployment and Access
---
<!-- font_size: 2 -->
This is entirely open source on Github, using this link: <span style="color: blue">https://github.com/LordGoatius/JIMNIAC</span>

Next Steps
---
<!-- font_size: 2 -->
The process for this was very long. My knowledge increased so fast while implementing this that
certain parts became obsolete as I was writing them.

For the next steps/designs I would:
- Use a RISC-V inspired design instead of x86
- Use smarter binary coded ternary representations
- Remove variable sized registers, instead using only 27 trit words
- Design for parallel graphics computation

Demo
---
<!-- font_size: 3 -->
<!-- alignment: center -->
GOTO demo
