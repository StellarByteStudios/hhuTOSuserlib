/* Runtime Environment. Übernommen zu großen Teilen aus D3OS */
pub mod environment;
#[cfg(feature = "runtime")] // Defaultfeature für Kernel deaktiviert -> allocator Dopplung
pub mod runtime;