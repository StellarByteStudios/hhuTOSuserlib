use core::panic::PanicInfo;
use crate::kernel::allocator::allocator::init;
use crate::kernel::syscall::user_api::{usr_get_pid, usr_hello_world_print};
use crate::kernel::runtime::environment;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap size

unsafe extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
}

#[cfg(feature = "lib-panic-handler")] // Defaultfeature für Kernel deaktiviert -> panic Handler Dopplung
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Hier bräuchte man noch einen noch einen guten Syscall
    usr_hello_world_print(69);
    loop { }
    /* TODO:
        - Syscall für kprint! -> Fehlermeldung
        - Statt loop einen Thread Exit
        */
}

// Entryfunktion die beim Starten der App angesprungen wird (Bereits Usermode)
#[link_section = ".entry"]
#[unsafe(no_mangle)]
extern "C" fn entry() {

    let pid: usize = usr_get_pid() as usize;
    init(pid, HEAP_SIZE);

    // Hier wird die Mainfunktion aufgerufen (Mit Parametern)
    unsafe {
        main(*environment::ARGC_PTR as isize, environment::ARGV_PTR);
    }

    // TODO: Beim return der Main den Prozess beenden (Syscall)
    //process::exit();
}