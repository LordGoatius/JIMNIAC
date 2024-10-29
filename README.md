# Overview
In this project I made a Harvard architecture virtual machine which operated on balanced ternary machine code.

`git checkout v2` to checkout my progress on my much more advanced von Neumann architecture machine.

# Operations
```
Bit	| ALU	| Branch | Stack | Flags
_____________________________________
LSH	| ADD	| CMP	 | PT	 |  C
RSH	| SUB	| BR	 | PTH	 |  -1, 0, or 1 based on previous results
NEG	| MUL	| BNE	 | PW	 |
AND	| ADD3	| BGT	 | PCT	 |
OR	| SUB3	| BLT	 | PCTH	 |
XOR	| MUL3	| BEQ	 | PCW	 |
	  ADD9
	  SUB9
	  MUL9
```

- P pushes to the data memory
- PC pushes to the instruction memory

These are all of the instructions in the current Ternary virtual machine
Its properties are odd, has no exact size for address space, and is overall incomplete.
But it is functional, so here is a guide.

# Assembly Language
You can use labels to do control flow. Execution begins at the first nonlabel instruction.
Example program:
```asm
main:
    PT 0t17
    PT 0t16
    CMP
    PCW label 
    BGT 
    ADD

label:
    PCW label2
    BR 
label2:
    PCW label
    BR 
```

There is no halt instruction, so just infinite loop instead. Turing failed to consider this in the halting problem I think.
