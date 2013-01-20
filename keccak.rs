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


; Define Rotation Offsets
#define ROT_OFFSETS #0x00001200
mov [ROT_OFFSETS+4], #1
mov [ROT_OFFSETS+8], #3
mov [ROT_OFFSETS+12], #6
mov [ROT_OFFSETS+16], #10
mov [ROT_OFFSETS+20], #15
mov [ROT_OFFSETS+24], #21
mov [ROT_OFFSETS+28], #28
mov [ROT_OFFSETS+32], #36
mov [ROT_OFFSETS+36], #45
mov [ROT_OFFSETS+40], #55
mov [ROT_OFFSETS+44], #2
mov [ROT_OFFSETS+48], #14
mov [ROT_OFFSETS+52], #27
mov [ROT_OFFSETS+56], #41
mov [ROT_OFFSETS+60], #56
mov [ROT_OFFSETS+64], #8
mov [ROT_OFFSETS+68], #25
mov [ROT_OFFSETS+72], #43
mov [ROT_OFFSETS+76], #62
mov [ROT_OFFSETS+80], #18
mov [ROT_OFFSETS+84], #39
mov [ROT_OFFSETS+88], #61
mov [ROT_OFFSETS+92], #20
mov [ROT_OFFSETS+96], #44

; Define Triangular Numbers
#define TRIANGLR_NUMS #0x00002096
mov [TRIANGLR_NUMS+4], #10
mov [TRIANGLR_NUMS+8], #7
mov [TRIANGLR_NUMS+12], #11
mov [TRIANGLR_NUMS+16], #17
mov [TRIANGLR_NUMS+20], #18
mov [TRIANGLR_NUMS+24], #3
mov [TRIANGLR_NUMS+28], #5
mov [TRIANGLR_NUMS+32], #16
mov [TRIANGLR_NUMS+36], #8
mov [TRIANGLR_NUMS+40], #21
mov [TRIANGLR_NUMS+44], #24
mov [TRIANGLR_NUMS+48], #4
mov [TRIANGLR_NUMS+52], #15
mov [TRIANGLR_NUMS+56], #23
mov [TRIANGLR_NUMS+60], #19
mov [TRIANGLR_NUMS+64], #13
mov [TRIANGLR_NUMS+68], #12
mov [TRIANGLR_NUMS+72], #2
mov [TRIANGLR_NUMS+76], #20
mov [TRIANGLR_NUMS+80], #14
mov [TRIANGLR_NUMS+84], #22
mov [TRIANGLR_NUMS+88], #9
mov [TRIANGLR_NUMS+92], #6
mov [TRIANGLR_NUMS+96], #1 

; Define Reference Constants
#define RC_BASE #0x00001000
mov [RC_BASE+4],   #0x00000001
mov [RC_BASE+8],   #0x00000001
mov [RC_BASE+12],  #0x00000000
mov [RC_BASE+16],  #0x00008082
mov [RC_BASE+20],  #0x80000000
mov [RC_BASE+24],  #0x0000808a
mov [RC_BASE+28],  #0x80000000
mov [RC_BASE+32],  #0x80008000
mov [RC_BASE+36],  #0x00000000
mov [RC_BASE+40],  #0x0000808b
mov [RC_BASE+44],  #0x00000000
mov [RC_BASE+48],  #0x80000001
mov [RC_BASE+52],  #0x80000000
mov [RC_BASE+56],  #0x80008081
mov [RC_BASE+60],  #0x80000000
mov [RC_BASE+64],  #0x00008009
mov [RC_BASE+68],  #0x00000000
mov [RC_BASE+72],  #0x0000008a
mov [RC_BASE+76],  #0x00000000
mov [RC_BASE+80],  #0x00000088
mov [RC_BASE+84],  #0x00000000
mov [RC_BASE+88],  #0x80008009
mov [RC_BASE+92],  #0x00000000
mov [RC_BASE+96],  #0x8000000a
mov [RC_BASE+100], #0x00000000
mov [RC_BASE+104], #0x8000808b
mov [RC_BASE+108], #0x80000000
mov [RC_BASE+112], #0x0000008b
mov [RC_BASE+116], #0x80000000
mov [RC_BASE+120], #0x00008089
mov [RC_BASE+124], #0x80000000
mov [RC_BASE+128], #0x00008003
mov [RC_BASE+132], #0x80000000
mov [RC_BASE+136], #0x00008002
mov [RC_BASE+140], #0x80000000
mov [RC_BASE+144], #0x00000080
mov [RC_BASE+148], #0x00000000
mov [RC_BASE+152], #0x0000800a
mov [RC_BASE+156], #0x80000000
mov [RC_BASE+160], #0x8000000a
mov [RC_BASE+164], #0x80000000
mov [RC_BASE+168], #0x80008081
mov [RC_BASE+172], #0x80000000
mov [RC_BASE+176], #0x00008080
mov [RC_BASE+180], #0x00000000
mov [RC_BASE+184], #0x80000001
mov [RC_BASE+188], #0x80000000
mov [RC_BASE+192], #0x80008008


#define INT_BC  #0x00003000 ; used internally


#define TEST_VECTOR #0x00002000
 ; our test vector for 24 round Keccak-256 "b0w.1z.1984&N0W"
 mov [TEST_VECTOR+#0],  #0xa8d71b07
 mov [TEST_VECTOR+#4],  #0xf4af26a4
 mov [TEST_VECTOR+#8],  #0xff21027f
 mov [TEST_VECTOR+#12], #0x62ff6026
 mov [TEST_VECTOR+#16], #0x7ff955c9
 mov [TEST_VECTOR+#20], #0x63f042c4
 mov [TEST_VECTOR+#24], #0x6da52ee3
 mov [TEST_VECTOR+#28], #0xcfaf3d3c

#define TEST_VECTOR_LEN #28 

; This number is not magic
; it is derived from 200 - (2 * Message Digest Length)
; where mdlen = 32, the mdlen of SHA-256
#define RSIZ #72 
#define RSIZW #9 ; RSIZ / 8 

; Keccak permutations are designated by keccak-f[b] where b defines the width of the
; permutation, the number of rounds depends on the width (in our case 1600, the highest)
; and is given by nr = 12 + 2l where 2^l = b / 25. This gives 24 rounds
#define KECCAK_ROUNDS #24

_start:
  call $keccak

keccak:
  sub r7, #28      ; allocate our space for our message digest
  sub r7, #50      ; allocate the length of the returned message (25 64 bit ints)
  mov r3, [r7+#50] ; Output buffer.
  sub r7, #144     ; allocate some temporary space
  
  ; Absorbing phase
  ; defined in case you need to change the size of your input vector
  ; forall block Pi in P
  ;   S[x,y] = S[x,y] xor Pi[x+5*y],          forall (x,y) such that x+5*y < r/w
  ;   S = Keccak-f[r+c](S)
  mov r0, RSIZ
  mov r1, TEST_VECTOR_LEN
  keccak_round: 
    push    r6   
    mov     r6, r7
    mov r2, #0
    xor_slice:
      ; xor twice because we've only got 32 bits of precision here
      ; and we are operating on 64 bit values, keep this in mind 
      xor [ROW_STATE + r2], [TEST_VECTOR+r2] 
      xor [ROW_STATE + r2 + #4], [TEST_VECTOR+r2+#4] 
      cmp r2, RSIZW
      jge xor_slice
    call $_keccak_round 
    sub r0, RSIZ 
    add r1, RSIZ
    cmp TEST_VECTOR_LEN, #24 ; rounds
    jge $keccak_round 
  mov     [VMADDR_NEWBLOCKPOS],  [ROW_STATE]   ; Pointer
  mov     [VMADDR_NEWBLOCKSIZE], TEST_VECTOR_LEN  ; Size
  call    $_success

_keccak_round:
  push    r6   
  mov     r6, r7
  call $theta
  push    r6   
  mov     r6, r7
  call $rho_pi
  push    r6   
  mov     r6, r7
  call $chi
  push    r6   
  mov     r6, r7
  call $iota
  ret

theta:
  call $parity
  ; C[x] = ROW_STATE[x,0] ⊕ OW_STATE[x,1] ⊕ ROW_STATE[x,2] ⊕ ROW_STATE[x,3] ⊕ ROW_STATE[x,4], ∀ x in 0...4
  ; D[x] = C[x - 1] ⊕ ROT(C[x + 1], 1),  ∀ x in 0...4
  call $theta_assignment
  ; ROW_STATE[x,y] = ROW_STATE[x,y] ⊕ D[x],                ∀ (x, y) in (0...4, 0...4)
  ret

; here's a haiku that describes this function 
; 32 bit word here
; standard calls for 64 bit
; xor them seperately
parity:
  mov r0, #0
  ; xor the lower 32 bits
  mov [INT_BC+r0], [ROW_STATE + r0]      
  xor [INT_BC+r0], [ROW_STATE + r0+#8]  
  xor [INT_BC+r0], [ROW_STATE + r0+#16]
  xor [INT_BC+r0], [ROW_STATE + r0+#24]  
  xor [INT_BC+r0], [ROW_STATE + r0+#32]  

  ; now xor the higher 32 bits
  mov [INT_BC+r0+#4], [ROW_STATE + r0+#4]  
  xor [INT_BC+r0+#4], [ROW_STATE + r0+#12]  
  xor [INT_BC+r0+#4], [ROW_STATE + r0+#20]
  xor [INT_BC+r0+#4], [ROW_STATE + r0+#28]  
  
  ; loop
  cmp r0, #5
  add r0, #1
  jnz $parity 
  ret


theta_assignment:
  push [r1+#4]
  push #5
  call $_mod

                         ; use the bitwise rotation to get through!
  push r6                ; save stack pointer
  mov r6, r7             ; create a new frame
  sub r7, #16            ; allocate 2 64 bit integros
  mov r2, r0
  push [r1+#8]
  push [r1+#4]
  push #1                ; push our arguments to our clever rotate function
  call $rotate

  xor r2, r0             ; r2 now contains an exclusive or of the mod and the rotation
  mov r0, #0x0           ; r0 is now j of the inner loop
  inner_theta_loop:
     add r0, r1
     xor [r6+#84+r0], r2
     pop r0
     cmp r0, #25
     mov r0, [r0+#5]
     jnz $inner_theta_loop
     ; jnz $_theta_assignment
     ret 
 
; INT_BC[y; 2x + 3y] = ROT(ROW_STATE[x; y]; r[x; y]), 8(x; y) in (0 : : : 4; 0 : : : 4)
rho_pi:
  pop r0 ; address of row state
  mov r1, #0 
  mov r4, [ROW_STATE + #8] ; 2nd item (dbl word precision)
  inner_pi:
    mov [INT_BC], #0x00000000
    ; iterate over the triangular numbers 0..24 
    mov r2, [TRIANGLR_NUMS + r1] 
    mov [INT_BC+#4], [ROW_STATE + r2]  
    mov [ROW_STATE + r2]
    
    push #0x00000000
    push r4
    mov r3, [ROT_OFFSETS + r1]
    push r3
    call $rotate
    pop r1 ; r1 now contains low value 
    pop r3 ; r3 now contains high value
    mov [ROW_STATE + r2], r3 
    mov [ROW_STATE + r2 + #4], r1
    mov r4, [INT_BC] 
    mov r4+#4, [INT_BC+#4]

    add r1, #1
    cmp r1, #24 
    jbe inner_pi 
  ret
 
; a[i][j][k] ⊕ = ¬a[i][j+1][k] & a[i][j+2][k].
chi:
  pop r0 ; address of row state 
  pop r1 ; bitwise combination pointer 
  ; iterate over all our rows 
  mov r2, #0
  outer_chi_loop:
    mov r3, #0 
    row_assignment:
      mov [r1+r3], r0[r2 + r3]     
      add r3, #1
      cmp r3, #5
      jbe row_assignment 
    mov r3, #0 
    bitwise_combine_along_rows:
      ; st[j + i] ^= (~bc[(i + 1) % 5]) & bc[(i + 2) % 5];
      cmp r3, #5
      jbe row_assignment 
  add r2, #5
  cmp r2, #25
  jbe outer_chi_loop
  ret

;  a[0,0] = a[0,0] xor RC
iota:
  pop r0 ; contains a pointer to the first value of our state
  pop r1 ; containts our round
  mov r2, #4 
  mul r2, r1
  xor [r0], r2
  xor [r0+#4], r2 + #4 ; unlimited references, wuw. 
  ret

     
; this function does bitwise rotation on a 64 bit value 
; with 32 bits of precision
; adapted from similar HACKMEM algorithm!
; ( mad respect from the youth of today! )
rotate:
  pop r0 ; r0 contains the count
  pop r1 ; r1 contains the low value
  pop r2 ; r2 contains the high value
  and r0, #0x3F
  cmp r0, #0x1F
  jbe inf32
  ; swap our values 
  mov r3, r1 
  mov r1, r2 
  mov r2, r3 
  and  r0, #0x1F
  inf32:
  ; hakmem magic ahead
  mov r5, 32
  sub r5, r0
  mov r4, r2
  shr r4, r5
  mov r4, r4
  mov r4, r1
  shl r4, r0
  or r4, r4
  shl r2, r0
  shr r1, r5
  or r2, r1
  mov r1, r4 
  push r1
  push r2
  ret
