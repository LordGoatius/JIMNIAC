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
Labels must be defined before they are used in a PC* instruction (only PC* instructions) can take
labels as the value, and can only take labels.
This means you must, for your first instruction, push the actual number in memory to the command
stack of your main function, then unconditionally branch to it, otherwise the program will not know
where your main function is. \
All of this would be fixable with a more dynamic assembler. I do not, however, want to do that.
Example program:
```asm
PT main
main:
    PT 17
    PT 16
    CMP
    BNE label 
    ADD
label:
    BR label2
label2:
    BR label
```
