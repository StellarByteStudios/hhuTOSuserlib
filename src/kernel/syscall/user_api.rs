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
    HelloWorldWithPrint, MMapHeapSpace, PaintPictureOnPos, PlaySongOnNoteList, KernelPrint,
    PrintAppNames, ExitThread, ExitProcess, KillProcess, DrawPixel, PrintRunningThreads,
    GetDateTime, GetPitInterval,
};
use core::arch::asm;
use crate::time::rtc_date_time::RtcDateTime;

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
pub fn usr_get_pid() -> u64 {
    return syscall(GetCurrentProcessID, &[]);
}
// Returned die Länge des gelesenen Namens
pub fn usr_read_process_name(buff: *mut u8, len: usize) -> u64 {
    return syscall(GetCurrentProcessName, &[buff as usize, len]);
}

pub fn usr_get_systime() -> u64 {
    return syscall(GetSystime, &[]);
}

pub fn usr_get_screen_width() -> u64 {
    return syscall(GetScreenWidth, &[]);
}

// Gibt die Startadresse des Heaps zurück
pub fn usr_mmap_heap_space(pid: usize, size: usize) -> u64 {
    return syscall(MMapHeapSpace, &[pid, size]);
}

pub fn usr_thread_exit() {
    syscall(ExitThread, &[]);
}

pub fn usr_process_exit() {
    syscall(ExitProcess, &[]);
}

pub fn usr_kill_process(pid: usize) {
    syscall(KillProcess, &[pid]);
}

pub fn usr_dump_active_vmas() {
    syscall(DumpVMAsOfCurrentProcess, &[]);
}

/*
    Ganz viel Printing
*/

pub fn usr_graphical_print(buff: *const u8, len: usize) {
    syscall(GraphicalPrint, &[buff as usize, len]);
}

pub fn usr_graphical_print_pos(x: usize, y: usize, buff: *const u8, len: usize) {
    syscall(GraphicalPrintWithPosition, &[x, y, buff as usize, len]);
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

pub fn usr_draw_pixel(x: usize, y: usize, color: usize) {
    syscall(DrawPixel, &[x, y, color]);
}

pub fn usr_kernel_print(buff: *const u8, len: usize) {
    syscall(KernelPrint, &[buff as usize, len]);
}

pub fn usr_print_all_apps() {
    syscall(PrintAppNames, &[]);
}

pub fn usr_print_running_thread() {
    syscall(PrintRunningThreads, &[]);
}

pub fn usr_play_song_with_notes(buff: *const u8, len: usize) {
    syscall(PlaySongOnNoteList, &[buff as usize, len]);
}

pub fn usr_get_datetime(datetime: *mut RtcDateTime) {
    syscall(GetDateTime, &[datetime as usize]);
}

// get pid interval in ms
pub fn usr_get_pid_interval() -> u64 {
    syscall(GetPitInterval, &[])
}
