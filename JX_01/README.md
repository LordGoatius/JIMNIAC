┌───────────────────────────────────────────────────────────────────────┐
│                       Machine Code / Opcodes                          │
├───────────────────────────────────────────────────────────────────────┤
│ CPU Specific Start with T00 (V), Stack / Branch / ALU with 0          │
│ [C]: Control Tribble [I]: Instr                                       │
│ [R]: Register                    Load               Branch            │
│                                  [R] = *imm         [PC] = [R] + imm  │
│ ALU:-[C][I]-[R][imm @ 3-8]       [R] = *([R] + imm) [PC] = [R] +      │
│            \[R][R][imm @ 4-8]                              [R] * imm  │
│                                  Store              Cmp               │
│ Stack Ops:      ALU Ops          *imm = [R]         [R] ~ imm         │
│ [R] + imm       [R] = [R] op imm *([R] + imm) = [R] [R] ~ [R] + imm   │
│ [R] + [R] * imm [R] = [R] op                                          │
│                       [R] * imm       lpt:  VP[R] x..                 │
│ int: VII [int] x..  egpu: VG[R] x..   intm: VIM [int] x..             │
│ dti: VB0 x..        lvb:  VL[R] imm.. inte: VIE [int] x..             │
│ sti: VS0 x..        egel: VX[R] x..   ints: VIS [int] x..             │
│ wfi: VW0 x..        pcsr: VC[0/A] x.. in:   IQ[R] [port] x..          │
│ rti: VR0 x..        ppsr: VD[0/A] x.. out:  IV[R] [port] x..          │
│ hlt: VM0 x..        pptr: VE[0/A] x..                                 │
│ lit: VA[R] x..        push/pop^ 1T0                                   │
└───────────────────────────────────────────────────────────────────────┘
┌────────────────┬───────┬────────┬─────────┐
│ CPU SPECIFIC:  │ ALU:  │ STACK: │ BRANCH: │
├────────────────┼───────┼────────┼─────────┤
│ hlt:           │ add A │ push K │ cmp  Q  │
│   halt         │       │        │         │
│ int:           │ sub B │ pop  L │ beq  R  │
│   interrupt    │       │        │         │
│ dti:           │ mul C │ call M │ bne  S  │
│    disable int │       │        │         │
│ sti:           │ qot D │ ret  N │ bgt  T  │
│   enable int   │       │        │         │
│ wfi:           │ rem E ├────────┤ blt  U  │
│   wait for int │       │ MEM:   │         │
│ rti:           │ and F ├────────┤ blq  V  │
│   return int   │       │        │         │
│ lit:           │ or  G │ load O │ bgq  W  │
│   loads idt    │       │        │         │
│ egpu:          │ sft H │ stre P │ bpz  X  │
│   enable gpu   │       │        │         │
│ lptb:          │ not I │        │ bpp  Y  │
│   load pg tbl  │       │        │         │
│ pcsr:          │ rot J │        │ bpn  Z  │
│   push csr     │       │        │         │
│ ppsr:          │       │        │         │
│   push psr     │       │        │         │
│ pptr:          │       │        │         │
│   push ptr     │       │        │         │
└────────────────┴───────┴────────┴─────────┘
