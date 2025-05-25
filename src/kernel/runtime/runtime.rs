use core::panic::PanicInfo;
use crate::kernel::allocator::allocator::init;
use crate::kernel::syscall::user_api::{usr_get_pid, usr_hello_world_print, usr_panic_print};
use crate::kernel::runtime::environment;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap size

unsafe extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
}

#[cfg(feature = "lib-panic-handler")] // Defaultfeature für Kernel deaktiviert -> panic Handler Dopplung
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // location der panic herausfiltern    
    let (file_ptr, file_len, line) = if let Some(loc) = info.location() {
        let file = loc.file().as_bytes();
        (file.as_ptr(), file.len(), loc.line())
    } else {
        (core::ptr::null(), 0, 0)
    };

    // panic message herausfiltern    
    let (msg_ptr, msg_len) = if let Some(msg) = info.message().as_str() {
        (msg.as_ptr(), msg.len())
    } else {
        (core::ptr::null(), 0)
    };    

    usr_panic_print(file_ptr, file_len, line as usize, msg_ptr, msg_len);

    loop { }
    /* TODO:        
        - Statt loop einen Thread Exit
        */
}

// Entryfunktion die beim Starten der App angesprungen wird (Bereits Usermode)
#[link_section = ".entry"]
#[unsafe(no_mangle)]
extern "C" fn entry() {

    //TODO: Heap Initialisierung in runtime verschieben
    // Allokator initialisieren
    let pid: usize = usr_get_pid() as usize;
    init(pid, HEAP_SIZE);

    // Hier wird die Mainfunktion aufgerufen (Mit Parametern)
    unsafe {
        main(*environment::ARGC_PTR as isize, environment::ARGV_PTR);
    }

    // TODO: Beim return der Main den Prozess beenden (Syscall)
    //process::exit();
}