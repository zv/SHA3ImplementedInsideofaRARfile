#include <constants.rh>
#include <crctools.rh>
#include <math.rh>
#include <util.rh>
; vim: syntax=fasm

; san francisco is covered over in darkness and it seems like everyone save yours truely is at ease in bed.
; it's been a cold and quiet weekend, so I thought I'd get back into hacking in assembly 
; so after much deliberation, rumination and something else young men do,
; here is the SHA-3 standard inside the RAR virtual machine filter language. here's my results.
; 10-08-2012 
; - zv

; the test vector for 32 round Keccak-256 
mov [r0+#0], #0xa8d71b07
mov [r0+#4], #0xf4af26a4
mov [r0+#8], #0xff21027f
mov [r0+#12], #0x62ff6026
mov [r0+#16], #0x7ff955c9
mov [r0+#20], #0x63f042c4
mov [r0+#24], #0x6da52ee3
mov [r0+#28], #0xcfaf3d3c


"Keccak-256 Test Hash"


keccak:
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



for (i = 0; i < 5; i++) {
            t = bc[(i + 4) % 5] ^ ROTL64(bc[(i + 1) % 5], 1);
            for (j = 0; j < 25; j += 5)
                st[j + i] ^= t;
        }

; me? i'm an assembly programmer.
_theta_assignment:
  push [r1+#4]
  push #5
  call $_mod
  ; the bitwise element swap, it does nothing! 
  ; i live dangerously close to spec 
  mov r0, r0 
  
  ; use the bitwise rotation to get through! 
  mov r2, r0
  push #1
  push [r1+#4]
  call $rotate
  xor r2, r0 ; r2 now contains t 
  mov r0, #0 ; r0 is now j of the inner loop
_inner_theta_loop: 
   add r0, r1
   xor [r6+#84+r0], r2  
   pop r0
   mod
   cmp r0, #25  
   mov r0, [r0+#5]
   jnz $inner_theta_loop
   jmp $rho_pi
   

; thanks HACKMEM! 
; mad respect from the youth of today!

rotate:
  ; (((x) << (y)) | ((x) >> (64 - (y))))
  push r6       ; save stack pointer
  mov r6, r7    ; create a new frame 
  sub r7, #16   ; allocate some, uhh, variables
  pop r0        ; x
  pop r1        ; y
  mov r2, r0    ; make a copy of x
  shl r0, r1    ; x << y
  add r1, -#64  ;(64 - y)
  shr r2, r1    ; x >> (64-y)
  or r0, r2     ; or those mugs
  pop r0 
  ret
;
;
;rho_and_pi:
;  mov r0, ST[1]
;  ; could make this a loop but honestly fuck that 
;  ; triangles are triangles, deal with it.
;   
;  mov r1, [ST+10] ; move triangular number
;  push r0 ; x
;  push #1 ; y
;  call rotate
;  
;  mov r0, r1
;  
_start:
