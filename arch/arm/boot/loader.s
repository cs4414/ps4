.text
.code 32
.syntax unified
.cpu arm926ej-s
.fpu softvfp

.global start
.global abort

.type start, %function

start:
    mov sp, 0x18000
    bl main
abort:
    b .
