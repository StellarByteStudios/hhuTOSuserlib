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
    GetScreenHeight,

    MMapHeapSpace,

    ExitThread,
    ExitProcess,
    KillProcess,

    DumpVMAsOfCurrentProcess,
    GraphicalPrint,
    GraphicalPrintWithPosition,
    PaintPictureOnPos,
    ClearScreen,

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

// Umwandlungsfunktion, welche einen Syscall aus einem usize erzeugt
impl core::convert::TryFrom<usize> for SystemCall {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        // Teste ob es ein valider Syscall ist
        if value < NUM_SYSCALLS {
            // Wandle in Syscall um und gebe ihn zurück
            return Ok(unsafe { core::mem::transmute(value) });
        }

        // Keine valide Syscallnummer
        return Err(());
    }
}
