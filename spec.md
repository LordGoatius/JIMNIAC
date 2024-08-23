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

Call, Return, and Divide macros will be provided (when I get to that)
Divding by 3 will be really easy though :).

FUTURE 27 Trits SYSTEM UPGRADES
---

Cycle Bitwise operator
"Hardware" floating point/fixed point support
Overflow flag, including negative or positive overflow
Branch based on sign of result
REGISTERS!!!!!!!!!!!!!!!!!!! (27 hopefully, most general purpopse)
And more when I think of it lol


 and| -1 |  0  |  1  
---------------------
 -1 |  1    0    -1
--------------------
 0  |  0    0     0
---------------------
 1  | -1    0     1
 
 or | -1 |  0  |  1  
---------------------
 -1 |  1   -1    0
--------------------
 0  | -1    0     1
---------------------
 1  |  0    1    -1


 xor = (a or b) and (not(a and b))

 xor| -1 |  0  |  1  
---------------------
 -1 | -1    0     0
--------------------
 0  |  0    0     0
---------------------
 1  |  0    0     1
