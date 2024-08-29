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

These are all of the instructions in the current Ternary virtual machine
Its properties are odd, has no exact size for address space, and is overall incomplete.
But it is functional, so here is a guide.

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

There is no halt instruction, so just infinite loop instead.
