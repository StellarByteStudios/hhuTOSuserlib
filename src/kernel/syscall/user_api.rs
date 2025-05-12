/*****************************************************************************
 *                                                                           *
 *                  u s e r _ a p i                                          *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Alle Systemaufrufe landen vom Assembler-Coder hier und   *
 *                  werden anhand der Funktionsnummerund der Funktions-      *
 *                  tabelle weitergeleitet.                                  *
 *                                                                           *
 * Autor:           Stefan Lankes, RWTH Aachen University                    *
 *                  Licensed under the Apache License, Version 2.0 or        *
 *                  the MIT license, at your option.                         *
 *                                                                           *
 *                  Michael Schoettner, 14.9.2023, modifiziert               *
 *****************************************************************************/

use core::arch::asm;

// Funktionsnummern aller Systemaufrufe
// TODO: Syscallnummern als Enum speichern
pub const SYSNO_HELLO_WORLD: usize = 0;
pub const SYSNO_HELLO_WORLD_PRINT: usize = 1;
pub const SYSNO_GET_LAST_KEY: usize = 2;
pub const SYSNO_GET_THREAD_ID: usize = 3;
pub const SYSNO_WRITE: usize = 4;
pub const SYSNO_READ: usize = 5;
pub const SYSNO_GET_SYSTIME: usize = 6;
pub const SYSNO_GRAPHICAL_PRINT: usize = 7;
pub const SYSNO_GRAPHICAL_PRINT_WITH_POS: usize = 8;
pub const SYSNO_GET_SCREEN_WIDTH: usize = 9;
pub const SYSNO_GET_PROCESS_ID: usize = 10;
pub const SYSNO_GET_PROCESS_NAME: usize = 11;
pub const SYSNO_DUMP_ACTIVE_VMAS: usize = 12;
pub const SYSNO_MMAP_HEAP_SPACE: usize = 13;
pub const SYSNO_PRINT_PICTURE: usize = 14;

pub const SYSNO_PLAY_SONG: usize = 15;

pub fn usr_hello_world() {
    syscall0(SYSNO_HELLO_WORLD as u64);
}

pub fn usr_hello_world_print(arg1: u64) {
    syscall1(SYSNO_HELLO_WORLD_PRINT as u64, arg1);
}

pub fn usr_getlastkey() -> u64 {
    return syscall0(SYSNO_GET_LAST_KEY as u64);
}

pub fn usr_gettid() -> u64 {
    return syscall0(SYSNO_GET_THREAD_ID as u64);
}

pub fn usr_write(buff: *const u8, len: u64) -> u64 {
    return syscall2(SYSNO_WRITE as u64, buff as u64, len);
}

pub fn usr_read(buff: *mut u8, len: u64) -> u64 {
    return syscall2(SYSNO_READ as u64, buff as u64, len);
}

pub fn usr_get_systime() -> u64 {
    return syscall0(SYSNO_GET_SYSTIME as u64);
}

pub fn usr_graphical_print(buff: *const u8, len: u64) {
    syscall2(SYSNO_GRAPHICAL_PRINT as u64, buff as u64, len);
}

pub fn usr_graphical_print_pos(x: u64, y: u64, buff: *const u8, len: u64) {
    syscall4(
        SYSNO_GRAPHICAL_PRINT_WITH_POS as u64,
        x,
        y,
        buff as u64,
        len,
    );
}

pub fn usr_get_screen_width() -> u64 {
    return syscall0(SYSNO_GET_SCREEN_WIDTH as u64);
}

pub fn usr_get_pid() -> u64 {
    return syscall0(SYSNO_GET_PROCESS_ID as u64);
}

// Returned die Länge des gelesenen Namens
pub fn usr_read_process_name(buff: *mut u8, len: u64) -> u64 {
    return syscall2(SYSNO_GET_PROCESS_NAME as u64, buff as u64, len);
}

pub fn usr_dump_active_vmas() {
    syscall0(SYSNO_DUMP_ACTIVE_VMAS as u64);
}

// Gibt die Startadresse des Heaps zurück
pub fn usr_mmap_heap_space(pid: usize, size: u64) -> u64 {
    return syscall2(SYSNO_MMAP_HEAP_SPACE as u64, pid as u64, size);
}

pub fn usr_paint_picture_on_pos(
    x: u64,
    y: u64,
    width: u64,
    height: u64,
    bbp: u64,
    bitmapbuff: *const u8,
) {
    syscall6(
        SYSNO_PRINT_PICTURE as u64,
        x,
        y,
        width,
        height,
        bbp,
        bitmapbuff as u64,
    );
}

pub fn usr_play_song(song_id: u64) {
    syscall1(SYSNO_PLAY_SONG as u64, song_id);
}


// TODO: Generischer syscall mit variablen Parametern
#[inline(always)]
#[allow(unused_mut)]
pub fn syscall0(arg0: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!("int 0x80",
            inlateout("rax") arg0 => ret,
            options(preserves_flags, nostack)
        );
    }
    ret
}

/*  Parameterreihenfolge
1. rdi
2. rsi
3. rdx
4. rcx
5. r8
6. r9
*/

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall1(arg0: u64, arg1: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
            "int 0x80", // Software interrupt for syscalls on x86_64 Linux
            in("rax") arg0,     // Load arg0 into rax (typically the syscall number)
            in("rdi") arg1,     // Load arg1 into rdi (first syscall parameter)
            lateout("rax") ret, // Store return value from syscall in ret
            options(preserves_flags, nostack)
        );
    }
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall2(arg0: u64, arg1: u64, arg2: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
            "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
            in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
            in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
            in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
            lateout("rax") ret,   // Store return value from syscall in ret
            options(preserves_flags, nostack)
        );
    }
    ret
}
#[inline(always)]
#[allow(unused_mut)]
pub fn syscall3(arg0: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
        in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
        in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
        in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
        in("rdx") arg3,       // Load arg3 into rdc (third syscall parameter)
        lateout("rax") ret,   // Store return value from syscall in ret
        options(preserves_flags, nostack)
        );
    }
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall4(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
        in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
        in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
        in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
        in("rdx") arg3,       // Load arg3 into rdc (third syscall parameter)
        in("rcx") arg4,       // Load arg4 into rdc (forth syscall parameter)
        lateout("rax") ret,   // Store return value from syscall in ret
        options(preserves_flags, nostack)
        );
    }
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall5(arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
        in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
        in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
        in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
        in("rdx") arg3,       // Load arg3 into rdc (third syscall parameter)
        in("rcx") arg4,       // Load arg4 into rdc (forth syscall parameter)
        in("r8")  arg5,
        lateout("rax") ret,   // Store return value from syscall in ret
        options(preserves_flags, nostack)
        );
    }
    ret
}

#[inline(always)]
#[allow(unused_mut)]
pub fn syscall6(
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    let mut ret: u64;
    unsafe {
        asm!(
        "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
        in("rax") arg0,       // Load arg0 into rax (typically the syscall number)
        in("rdi") arg1,       // Load arg1 into rdi (first syscall parameter)
        in("rsi") arg2,       // Load arg2 into rsi (second syscall parameter)
        in("rdx") arg3,       // Load arg3 into rdc (third syscall parameter)
        in("rcx") arg4,       // Load arg4 into rdc (forth syscall parameter)
        in("r8")  arg5,
        in("r9")  arg6,
        lateout("rax") ret,   // Store return value from syscall in ret
        options(preserves_flags, nostack)
        );
    }
    ret
}
