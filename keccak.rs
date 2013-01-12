#include <constants.rh>
#include <crctools.rh>
#include <math.rh>
#include <util.rh>
; vim: syntax=fasm
; 10-08-2012 
; - zv
; this is a pure RAR file virtual machine implementation of the new SHA-3 standard keccak
; this code is licensed under microsofts code freedom license
; just kidding
; MIT LICENSE - FREE FOR REDISTRIBUTION

_start:
  call $keccak

keccak:
  sub r7, #28      ; allocate our space for our message digest
  sub r7, #50      ; allocate the length of the returned message (25 64 bit ints)
  mov r3, [r7+#50] ; Output buffer.
  sub r7, #144     ; allocate some temporary space

   ; the test vector for 24 round Keccak-256 "Keccak-256 Test Hash"
   mov [r0+#0],  #0xa8d71b07
   mov [r0+#4],  #0xf4af26a4
   mov [r0+#8],  #0xff21027f
   mov [r0+#12], #0x62ff6026
   mov [r0+#16], #0x7ff955c9
   mov [r0+#20], #0x63f042c4
   mov [r0+#24], #0x6da52ee3
   mov [r0+#28], #0xcfaf3d3c

  ; Keccak permutations are designated by keccak-f[b] where b defines the width of the
  ; permutation, the number of rounds depends on the width (in our case 1600, the highest)
  ; and is given by nr = 12 + 2l where 2^l = b / 25. This gives 24 rounds
  cmp r0, #24
  call $keccak_round
  xor r0, r0
  mov r0, #0x0 
  add r0, #0x1 
 
keccak_round:
  call $theta
  call $rho_pi
  call $chi
  call $iota
  
theta:
  call $parity
  call $theta_column_assignment 

rho_pi:
  call $rotate_by_triangular_number
  
chi:
  call $bitwise_combine_along_rows
  
iota:
  call $lsfr

; here's a haiku that describes this function 
; 32 bit word here
; standard calls for 64 bit
; xor them seperately
parity:
  mov r1, [r0]      ; set the lower value of bc[i]
  xor r1, [r0+64]   ; now xor the lower 32 bits
  xor r1, [r0+128]
  xor r1, [r0+192]  
  xor r1, [r0+256]  
  mov [r6+#4+r4], r1 

  mov r1, [r0+32]  ; set the upper value of bc[i] 
  xor r1, [r0+96]  ; now xor the higher 32 bits
  xor r1, [r0+160]
  xor r1, [r0+288]  
  mov [r6+#8+r4], r1 
  
  ; loop
  cmp r2, #5
  add r2, #1
  jnz $parity 
  
  ret

_rho_assignment:
  push r6 
  mov r6, r7 
  sub r7, #16 ; just dealing with the shit in order to be the shit
  ; fuck it
  ; this is it
  ; it is what it is

_theta_assignment:
  push r6      
  mov r6, r7    
  sub r7, #16   ; allocate some, uhh, variables
  push [r1+#4]
  push #5
  call $_mod
  mov r0, r0 ; i live dangerously close to spec 
  
  ; use the bitwise rotation to get through! 
  push r6       ; save stack pointer
  mov r6, r7    ; create a new frame 
  sub r7, #16   ; allocate 2 64 bit integros 
  mov r2, r0   
  push #1       ; first argument to rotate
  push [r1+#4]  ; second argument (i + 4) 
  call $rotate  ; using the boost to get through!
  
  xor r2, r0 ; r2 now contains an exclusive or of the mod and the rotation  
  mov r0, #0 ; r0 is now j of the inner loop
_inner_theta_loop: 
   add r0, r1
   xor [r6+#84+r0], r2  
   pop r0
   cmp r0, #25  
   mov r0, [r0+#5]
   jnz $inner_theta_loop
   ; jnz $_theta_assignment   
   jmp $rho_pi
  
 

;    10, 7,  11, 17, 18, 3, 5,  16, 8,  21, 24, 4, 
;    15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6,  1 


rho_pi:
; this is so ghetto
; but then again so am i
;
; the main insight of this algorithm is to take some value out of 
; the positional ilngt, rotl it and modify the original string supplied
; to the so called "algorithm" 
;  push r6     ; you know what this does
;  mov r6, r7  ; activate the frames batman!
;       t = st[1];
;        for (i = 0; i < 24; i++) {
;            j = keccakf_piln[i];
;            bc[0] = st[j];
;            st[j] = ROTL64(t, keccakf_rotc[i]);
;            t = bc[0];
;        }


  mov r0, ST[1]
   
  mov r1, [ST+10] ; move triangular number
  push r0 ; x
  push #1 ; y
  call rotate
  
  mov r0, r1


; thanks HACKMEM! 
; mad respect from the youth of today!
_rotate:
  and count, 0x3F
  cmp count, 0x1F
  jbe inf32
  mov tmp, low
  mov low, high
  mov high, tmp
  and count, 0x1F
  inf32:
  mov tmpcount, 32
  sub tmpcount, count
  mov tmp2, high
  shr tmp2, tmpcount
  mov tmp, tmp2
  mov tmp2, low
  shl tmp2, count
  or tmp, tmp2
  shl high, count
  shr low, tmpcount
  or high, low
  mov low, tmp2 
  

  
_start:
