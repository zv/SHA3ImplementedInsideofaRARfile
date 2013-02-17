## Overview of SHA-3

At the opening of this new millenium, it was discovered that our most-used hash 
functions fell ridiculously short of the ideal hash function.

In the past few years, researchers published an active break on the SSL 
protocol: their attack used the weakness of the popular MD5 algorithm to enable 
them to "lift" a digital signature from one document to another, since SSL only 
signs the document's hash, not the whole document. (SSL certificates contain a 
little section saying how much you can trust them. When they "lifted" the 
signature, they made sure that their destination document was a certificate 
which said "you can trust me 100%" -- they used this to break *all of SSL*, and
not just one pithy website.)

MD5 is still used by some, but in real applications it has mostly been replaced
by an algorithm that the US National Security Administration published, called
SHA-1. Unfortunately, SHA-1 has now been pushed to a very uncomfortable level.
People are worried because the NSA's updated algorithms -- the SHA-2 algorithms 
(sha224, sha256, sha384, and sha512) -- have the same general structure as 
SHA-1 did. 

The US government had done reasonably well when it last orchestrated a public
cryptographic competition: the block cipher that they created, AES, is now the
<i>de facto</i> standard for all secure information transmission online, as well 
as one of the popular options for hard-drive encryption. It has a couple of 
breaks at this point, especially on its 256-bit key version, but it's nothing 
too scary. The US agency NIST decided to organize a new contest, this time for a 
next-generation hash function. Though it will have nothing to do with either 
SHA-1 or SHA-2, they have insisted on the confusing and unimaginative name 
"SHA-3". We can only hope that they change their minds before the contest is 
over. It is bad enough that nobody calls the SHA-2 algorithms by the name
"SHA-2;" we do not need to make it worse. 

The fantastic success of their last competition was not forgotten: they 
received some 60-something contest entries, many of which had pretty bad flaws 
hiding beneath the surface. To give the researchers a smaller target group to 
focus more heavily on, they reduced it to just 14 candidates. The contest is 
not over yet, but these 14 candidates are all pretty serious contenders, and
everyone wants to poke holes in everyone else's submissions.

# SHA3 inside a RAR VM 

So where do we go from here? How about.... Keccak inside the RARVM?


# RAR VM?
Believe it or not, RAR files can contain bytecode for a simple x86-like virtual machine called the RarVM. 
This is designed to provide filters (preprocessors) to perform some reversible transformation on input data 
to increase redundancy, and thus improve compression.

For example, one filter (likely inspired by LZX, an earlier scheme with a similar feature) is called "Intel E8 preprocessing",
which is designed to increase redundancy in x86 code.

WinRAR includes around a dozen standard filters that improve compression of several common inputs, but surprisingly also allows new filters to be defined at runtime by archives!

(A related blog post is available here http://blog.cmpxchg8b.com/2012/09/fun-with-constrained-programming.html)

Architecture
===============================================================================

Familiarity with x86 (and preferably intel assembly syntax) would be an advantage.

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

