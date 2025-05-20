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

use crate::kernel::syscall::SystemCall::{self,
    DumpVMAsOfCurrentProcess, GetCurrentProcessID, GetCurrentProcessName, GetCurrentThreadID,
    GetLastKey, GetScreenWidth, GetSystime, GraphicalPrint, GraphicalPrintWithPosition, HelloWorld,
    HelloWorldWithPrint, MMapHeapSpace, PaintPictureOnPos, PlaySong, DeleteLastScreenChars, 
};
use core::arch::asm;

// TODO: Generischer syscall mit variablen Parametern
// Inspired by D3OS
// Generischer Syscall. Kann für alle Argumentanzahlen verwendet werden, die unterstützt werden
pub fn syscall(call: SystemCall, args: &[usize]) -> u64 {
    let mut ret: u64;

    // Abfrage ob nicht zu viele Argumente übergeben wurden
    if args.len() > 6 {
        panic!("System calls with more than 6 params are not supported.");
    }

    // Alle Argumente laden. Wenn nicht vorhanden einfach null-Referenz (sollte eh nicht benutz werden)
    let arg0 = *args.first().unwrap_or(&0usize);
    let arg1 = *args.get(1).unwrap_or(&0usize);
    let arg2 = *args.get(2).unwrap_or(&0usize);
    let arg3 = *args.get(3).unwrap_or(&0usize);
    let arg4 = *args.get(4).unwrap_or(&0usize);
    let arg5 = *args.get(5).unwrap_or(&0usize);

    unsafe {
        asm!(
        "int 0x80",           // Software interrupt for syscalls on x86_64 Linux
        in("rax") call as u64,// Load call into rax (the syscall number)
        in("rdi") arg0,       // Load arg0 into rdi (first syscall parameter)
        in("rsi") arg1,       // Load arg1 into rsi (second syscall parameter)
        in("rdx") arg2,       // Load arg2 into rdc (third syscall parameter)
        in("rcx") arg3,       // Load arg3 into rdc (forth syscall parameter)
        in("r8")  arg4,
        in("r9")  arg5,
        lateout("rax") ret,   // Store return value from syscall in ret
        options(preserves_flags, nostack)
        );
    }

    return ret;
}

pub fn usr_hello_world() {
    syscall(HelloWorld, &[]);
}

pub fn usr_hello_world_print(arg1: usize) {
    syscall(HelloWorldWithPrint, &[arg1]);
}

pub fn usr_getlastkey() -> u64 {
    return syscall(GetLastKey, &[]);
}

pub fn usr_gettid() -> u64 {
    return syscall(GetCurrentThreadID, &[]);
}

pub fn usr_get_systime() -> u64 {
    return syscall(GetSystime, &[]);
}

pub fn usr_graphical_print(buff: *const u8, len: usize) {
    syscall(GraphicalPrint, &[buff as usize, len]);
}

pub fn usr_graphical_print_pos(x: usize, y: usize, buff: *const u8, len: usize) {
    syscall(GraphicalPrintWithPosition, &[x, y, buff as usize, len]);
}

pub fn usr_get_screen_width() -> u64 {
    return syscall(GetScreenWidth, &[]);
}

pub fn usr_get_pid() -> u64 {
    return syscall(GetCurrentProcessID, &[]);
}

// Returned die Länge des gelesenen Namens
pub fn usr_read_process_name(buff: *mut u8, len: usize) -> u64 {
    return syscall(GetCurrentProcessName, &[buff as usize, len]);
}

pub fn usr_dump_active_vmas() {
    syscall(DumpVMAsOfCurrentProcess, &[]);
}

// Gibt die Startadresse des Heaps zurück
pub fn usr_mmap_heap_space(pid: usize, size: usize) -> u64 {
    return syscall(MMapHeapSpace, &[pid, size]);
}

pub fn usr_paint_picture_on_pos(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    bbp: usize,
    bitmapbuff: *const u8,
) {
    syscall(
        PaintPictureOnPos,
        &[x, y, width, height, bbp, bitmapbuff as usize],
    );
}

pub fn usr_play_song(song_id: usize) {
    syscall(PlaySong, &[song_id]);
}

pub fn usr_delete_last_screen_chars(n: usize) {
    syscall(DeleteLastScreenChars, &[n]);
}
/*
pub fn usr_hello_world() {
    syscall0(HelloWorld as u64);
}

pub fn usr_hello_world_print(arg1: u64) {
    syscall1(HelloWorldWithPrint as u64, arg1);
}

pub fn usr_getlastkey() -> u64 {
    return syscall0(GetLastKey as u64);
}

pub fn usr_gettid() -> u64 {
    return syscall0(GetCurrentThreadID as u64);
}

/* Wird erstmal rausgeschmissen
pub fn usr_write(buff: *const u8, len: u64) -> u64 {
    return syscall2(SYSNO_WRITE as u64, buff as u64, len);
}

pub fn usr_read(buff: *mut u8, len: u64) -> u64 {
    return syscall2(SYSNO_READ as u64, buff as u64, len);
}*/

pub fn usr_get_systime() -> u64 {
    return syscall0(GetSystime as u64);
}

pub fn usr_graphical_print(buff: *const u8, len: u64) {
    syscall2(GraphicalPrint as u64, buff as u64, len);
}

pub fn usr_graphical_print_pos(x: u64, y: u64, buff: *const u8, len: u64) {
    syscall4(GraphicalPrintWithPosition as u64, x, y, buff as u64, len);
}

pub fn usr_get_screen_width() -> u64 {
    return syscall0(GetScreenWidth as u64);
}

pub fn usr_get_pid() -> u64 {
    return syscall0(GetCurrentProcessID as u64);
}

// Returned die Länge des gelesenen Namens
pub fn usr_read_process_name(buff: *mut u8, len: u64) -> u64 {
    return syscall2(GetCurrentProcessName as u64, buff as u64, len);
}

pub fn usr_dump_active_vmas() {
    syscall0(DumpVMAsOfCurrentProcess as u64);
}

// Gibt die Startadresse des Heaps zurück
pub fn usr_mmap_heap_space(pid: usize, size: u64) -> u64 {
    return syscall2(MMapHeapSpace as u64, pid as u64, size);
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
        PaintPictureOnPos as u64,
        x,
        y,
        width,
        height,
        bbp,
        bitmapbuff as u64,
    );
}

pub fn usr_play_song(song_id: u64) {
    syscall1(PlaySong as u64, song_id);
}

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
*/
