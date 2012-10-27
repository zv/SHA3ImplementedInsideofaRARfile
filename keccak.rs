#include <constants.rh>
#include <crctools.rh>
#include <math.rh>
#include <util.rh>
; vim: syntax=fasm



; shit just got real.

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


; [r6+4] beginning of bc
; [r6+84] beginning of st

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
  
parity:
  ; set the lower value of bc[i]
  mov r1, [r0]
  xor r1, [r0+64]  
  xor r1, [r0+128]
  xor r1, [r0+192]  
  xor r1, [r0+256]  
  mov [r6+#4+r4], r1 

 ; set the upper value of bc[i] 
  mov r1, [r0+32]
  xor r1, [r0+96]  
  xor r1, [r0+160]
  xor r1, [r0+288]  
  mov [r6+#8+r4], r1 

  cmp r2, #5
  add r2, #1
  jnz $parity 


;theta_assignment:
;  push [r1+#4]
;  push #5
;  ; one day i will be cool enough to implement mod myself 
;  call $_mod
;  ; grab the shit out of the bc  and store in r1
;  ; r6+4+r0; this is bc[i+4 % 5]
;  ; the bitwise element swap, it does nothing!
;  mov r0, r0 
;  
;  ; use the bitwise rotation to get through! 
;  mov r2, r0
;  push #1
;  push [r1+#4]
;  call $rotate
;  xor r2, r0 ; r2 now contains t 
;  mov r0, #0 ; r0 is now j of the inner loop
;  jnz $inner_theta_loop 
;  
;inner_theta_loop: 
;   add r0, r1
;   xor [r6+#84+r0], r2  
;   pop r0
;   mod
;   cmp r0, #25  
;   mov r0, [r0+#5]
;   jnz $inner_theta_loop
;   jmp $rho_pi
;   
;   
;
;
;rotate:
;  ; (((x) << (y)) | ((x) >> (64 - (y))))
;  pop r0 ; x
;  pop r1 ; y
;  mov r2, r0 ; make a copy of x
;  shl r0, r1 ; x << y
;  shr r2, [#64-r1] ; x >> (64-y)
;  or r0, r2 
;  pop r0 
;  ret
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
