pub mod allocator;

#[cfg(feature = "kprint")] // Defaultfeature für Kernel deaktiviert -> Dopplung im Kernel
#[macro_use]
pub mod kprint;
pub mod runtime;
pub mod shell;
pub mod syscall;
