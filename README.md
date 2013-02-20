# SHA3 in a RAR file

It's old hat now but NIST has settled on Keccak as the upcoming SHA-3 Hash function standard.

In celebration of keccak's standardization, here's an implementation based on the specification implementation [Here](http://keccak.noekeon.org/KeccakTools-doc/index.html) 
that takes some arbitrary number of bytes and a size variable (pascal style!) and produces a Keccak-256 hash  (64 bytes) 

You can build it by updating the rarvmtools git submodule and `make`

## RAR VM?
Believe it or not, RAR files can contain bytecode for a simple x86-like virtual machine called the RarVM. 
This is designed to provide filters (preprocessors) to perform some reversible transformation on input data 
to increase redundancy, and thus improve compression.

For example, one filter (likely inspired by LZX, an earlier scheme with a similar feature) is called "Intel E8 preprocessing",
which is designed to increase redundancy in x86 code.

WinRAR includes around a dozen standard filters that improve compression of several common inputs, but surprisingly also allows new filters to be defined at runtime by archives!

(Find out more here --  http://blog.cmpxchg8b.com/2012/09/fun-with-constrained-programming.html)


## How do I shot web?

Familiarity with x86 (and preferably intel assembly syntax) is basically required. 

RarVM has 8 named registers, called r0 to r7. r7 is used as a stack pointer for
stack related operations (such as push, call, pop, etc). However, as on x86,
there are no restrictions on setting r7 to whatever you like, although if you
do something stack related it will be masked to fit within the address space
for the duration of that operation.

A RarVM program may execute for at most 250,000,000 instructions, at which
point it will be terminated abnormally. However there are no limits on the
number of programs included in a file, so you must simply split your task into
multiple 250M cycle chunks. If the instruction pointer ever moves
outside the defined code segment, the program is considered to have completed
successfully, and is terminated normally.

Your program will execute with a 0x40000 byte address space, and has access to
3 status flags : ZF (Zero), CF (Carry) and SF (Sign) which can be accessed via
the conditional branch instructions, or via pushf/popf (as on x86).


Test vectors are provided in the source code to prove compliance wherever 
possible.

[keccak]: http://keccak.noekeon.org/ "Keccak Homepage"

