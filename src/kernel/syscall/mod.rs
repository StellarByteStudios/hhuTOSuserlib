pub mod process_management;
pub mod user_api;

// Enum with all known system calls
// Inspired by D3OS
#[repr(usize)]
#[allow(dead_code)]
pub enum SystemCall {
    HelloWorld = 0,
    HelloWorldWithPrint,
    GetLastKey,

    GetCurrentThreadID,
    GetCurrentProcessID,
    GetCurrentProcessName,

    GetSystime,
    GetScreenWidth,

    MMapHeapSpace,

    ExitThread,
    ExitProcess,
    KillProcess,

    DumpVMAsOfCurrentProcess,
    GraphicalPrint,
    GraphicalPrintWithPosition,
    PaintPictureOnPos,

    KernelPrint,
    PrintAppNames,
    PrintRunningThreads,

    PlaySongOnNoteList,

    DrawPixel,
    GetDateTime,
    GetPitInterval,

    ActivateShell,
    DeactivateShell,

    // kein Syscall. Speichert aber dadurch die Anzahl der Syscalls
    LastEntryMarker,
}
pub const NUM_SYSCALLS: usize = SystemCall::LastEntryMarker as usize;
