//===----------------------------------------------------------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is dual licensed under the MIT and the University of Illinois Open
// Source Licenses. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//

        .syntax unified
        .cpu    arm926ej-s

.globl __aeabi_memset
__aeabi_memset:
        mov     r3, r1
        mov     r1, r2
        mov     r2, r3
        b       memset

.globl __aeabi_memcpy
__aeabi_memcpy:
        b       memcpy

.globl __aeabi_memmove
__aeabi_memmove:
        b memmove

        .align 2
.globl __aeabi_ldivmod
__aeabi_ldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __divmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}

        .align 2
.globl __aeabi_uldivmod
__aeabi_uldivmod:
        push    {r11, lr}
        sub     sp, sp, #16
        add     r12, sp, #8
        str     r12, [sp]
        bl      __udivmoddi4
        ldr     r2, [sp, #8]
        ldr     r3, [sp, #12]
        add     sp, sp, #16
        pop     {r11, pc}

.align 3
 .globl __aeabi_uidiv
 .globl __udivsi3
__aeabi_uidiv:
 __udivsi3:
# 51 "udivsi3.S"
    push {r7, lr} ; mov r7, sp
    clz r2, r0
    tst r1, r1
    clz r3, r1
    mov ip, #0
    beq .L_return
    mov lr, #1
    subs r3, r3, r2
    blt .L_return

.L_mainLoop:
# 75 "udivsi3.S"
    subs r2, r0, r1, lsl r3
    itt hs
    orrhs ip, ip,lr, lsl r3
    movhs r0, r2
    it ne
    subsne r3, r3, #1
    bhi .L_mainLoop



    subs r2, r0, r1
    it hs
    orrhs ip, #1

.L_return:

    mov r0, ip
    pop {r7, pc}

.align 3
.globl __umodsi3
 __umodsi3:
# 39 "umodsi3.S"
    clz r2, r0
    tst r1, r1
    clz r3, r1
    bxeq lr
    subs r3, r3, r2
    bxlt lr

.L_mainLoop2:
# 59 "umodsi3.S"
    subs r2, r0, r1, lsl r3
    it hs
    movhs r0, r2
    it ne
    subsne r3, r3, #1
    bhi .L_mainLoop2



    subs r2, r0, r1
    it hs
    movhs r0, r2
    bx lr

.align 3
.globl __aeabi_idiv
__aeabi_idiv:
__divsi3:
# 37 "divsi3.S"
push {r4, r7, lr} ; add r7, sp, #4

    eor r4, r0, r1

    eor r2, r0, r0, asr #31
    eor r3, r1, r1, asr #31
    sub r0, r2, r0, asr #31
    sub r1, r3, r1, asr #31

    bl __udivsi3

    eor r0, r0, r4, asr #31
    sub r0, r0, r4, asr #31
    pop {r4, r7, pc}

.align 3
 ; .globl __modsi3 ; __modsi3:
# 36 "modsi3.S"
    push {r4, r7, lr} ; add r7, sp, #4

    mov r4, r0

    eor r2, r0, r0, asr #31
    eor r3, r1, r1, asr #31
    sub r0, r2, r0, asr #31
    sub r1, r3, r1, asr #31

    bl __umodsi3

    eor r0, r0, r4, asr #31
    sub r0, r0, r4, asr #31
    pop {r4, r7, pc}

// https://android.googlesource.com/platform/bionic/+/884e4f8/libc/arch-arm/bionic/memset.S
/*
 * Copyright (C) 2008 The Android Open Source Project
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 * OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED
 * AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

    /*
     * Optimized memset() for ARM.
     *
     * memset() returns its first argument.
     */
memset:
        /* compute the offset to align the destination
         * offset = (4-(src&3))&3 = -src & 3
         */
        .fnstart
        .save       {r0, r4-r7, lr}
        stmfd       sp!, {r0, r4-r7, lr}
        rsb         r3, r0, #0
        ands        r3, r3, #3
        cmp         r3, r2
        movhi       r3, r2

        /* splat r1 */
        mov         r1, r1, lsl #24
        orr         r1, r1, r1, lsr #8
        orr         r1, r1, r1, lsr #16

        movs        r12, r3, lsl #31
        strcsb      r1, [r0], #1    /* can't use strh (alignment unknown) */
        strcsb      r1, [r0], #1
        strmib      r1, [r0], #1
        subs        r2, r2, r3
        ldmlsfd     sp!, {r0, r4-r7, lr}   /* return */
        bxls        lr

        /* align the destination to a cache-line */
        mov         r12, r1
        mov         lr, r1
        mov         r4, r1
        mov         r5, r1
        mov         r6, r1
        mov         r7, r1

        rsb         r3, r0, #0
        ands        r3, r3, #0x1C
        beq         3f
        cmp         r3, r2
        andhi       r3, r2, #0x1C
        sub         r2, r2, r3

        /* conditionnaly writes 0 to 7 words (length in r3) */
        movs        r3, r3, lsl #28
        stmcsia     r0!, {r1, lr}
        stmcsia     r0!, {r1, lr}
        stmmiia     r0!, {r1, lr}
        movs        r3, r3, lsl #2
        strcs       r1, [r0], #4

3:
        subs        r2, r2, #32
        mov         r3, r1
        bmi         2f
1:      subs        r2, r2, #32
        stmia       r0!, {r1,r3,r4,r5,r6,r7,r12,lr}
        bhs         1b
2:      add         r2, r2, #32

        /* conditionnaly stores 0 to 31 bytes */
        movs        r2, r2, lsl #28
        stmcsia     r0!, {r1,r3,r12,lr}
        stmmiia     r0!, {r1, lr}
        movs        r2, r2, lsl #2
        strcs       r1, [r0], #4
        strmih      r1, [r0], #2
        movs        r2, r2, lsl #2
        strcsb      r1, [r0]
        ldmfd       sp!, {r0, r4-r7, lr}
        bx          lr
