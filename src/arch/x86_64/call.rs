// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

// x86-64 ABI/calling convention of syscall
//
// | register | usage    |
// |----------|----------|
// | rax      | call num | also use for store the return value.
// | rdi      | 1st      |
// | rsi      | 2nd      |
// | rdx      | 3rd      |
// | r10      | 4th      | !! 'rcx' for standard function calling
// | r8       | 5th      |
// | r9       | 6th      |
//
// arguments over 6 are passed through the stack, as well as the
// variable arguments (the amount of arguments are variable).
//
// /-------------------------\ <-- stack start
// |   ...                   |
// | 8th arg                 | <-- rbp + 24
// | 7th arg                 | <-- rbp + 16
// | previous rip (ret addr) | <-- rbp + 8
// | previous rbp            | <-- rbp
// | saved regs e.g. r12-r15 |
// |-------------------------| <-- rsp
// |   ...                   |
// | 128 bytes               | reserved area (red zone) for such as local variables
// |-------------------------|
// |                         |

// r10, r11 for temporary, as well as the registers above are not preserved
// across a function call.
//
// registers: rax, rdi, rsi, rdx, rcx,    r10,    and r8, r9, 11 are scratch (caller saved) registers,
//            ---  ---  ---  ---  ---     ---         --  --
// arguments: ret  a1   a2   a3   a4(std) a4(syscall) a5  a6
//
// any of these registers may be used in a function without have to save the original value.
// this also means that you need to save them BEFORE invoking a function call
// if you need the value of one of these registers.
// to keep things simple, just do not use scratch registers to hold the 'long live' values.
//
// on the other hand, rbx, rsp, rbp, r12-15 are preserved (callee saved) registers, this
// means that when you generate a function, you need to save these register on the stack
// BEFORE using them and MUST RESTORE them before returning.
//
// ref: https://www.cs.uaf.edu/2017/fall/cs301/reference/x86_64.html
// ref: https://www.cs.fsu.edu/~langley/CNT5605/2017-Summer/assembly-example/assembly.html

// note:
//
// `rcx` and `r11` are used for store the values of `rip` and `rflags` register at syscall,
// these are done automatictly by kernel.
// when syscall is finish, the old values of `rcx` and `r11` would not be restored,
// therefor, to keep things simple, it is better to mark them as 'out' in the inline ASM
// to avoid the callee to use them.
//
// https://supercip971.github.io/02-wingos-syscalls.html

// syscall example: print "hello world" to stdout
//
// ```yasm
// STDOUT_FILENO    equ     1
// SYS_write        equ     1
// message          db      "Hello, World!"
// message_length   dq      13
//
// mov              rax,    SYS_write
// mov              rdi,    STDOUT_FILENO
// mov              rsi,    message
// mov              rdx,    qword [message_length]
// syscall
// ```
//
// ref:
// - YASM
//   https://yasm.tortall.net/
// - Rust inline assembly
//   https://doc.rust-lang.org/stable/reference/inline-assembly.html
use std::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_without_args(num: usize) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_1_arg(num: usize, arg1: usize) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_2_args(num: usize, arg1: usize, arg2: usize) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_3_args(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_4_args(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_5_args(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall_with_6_args(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> Result<usize, usize> {
    let mut result: isize;
    asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8") arg5,
        in("r9") arg6,
        out("rcx") _,
        out("r11") _,
        lateout("rax") result,
        options(nostack, preserves_flags)
    );
    convert_raw_return_code_from_rax(result)
}

#[inline(always)]
fn convert_raw_return_code_from_rax(raw_code: isize) -> Result<usize, usize> {
    if raw_code < 0 {
        Err((-raw_code) as usize)
    } else {
        Ok(raw_code as usize)
    }
}
