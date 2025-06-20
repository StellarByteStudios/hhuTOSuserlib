use crate::kernel::allocator::allocator::init;
use crate::kernel::runtime::environment;
use crate::kernel::syscall::user_api::{usr_get_pid, usr_process_exit, usr_thread_exit};
use alloc::string::{String, ToString};
use core::panic::PanicInfo;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap size

unsafe extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
}

#[cfg(feature = "lib-panic-handler")] // Defaultfeature für Kernel deaktiviert -> panic Handler Dopplung
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut panic_message = String::from("Panic: ");

    // panic message herausfiltern
    if let Some(msg) = info.message().as_str() {
        panic_message.push_str(msg);
    } else {
        panic_message.push_str("no panic message");
    };

    // location der panic herausfiltern
    if let Some(loc) = info.location() {
        panic_message.push_str(" at ");
        panic_message.push_str(loc.file());
        panic_message.push(':');
        let line = loc.line();
        panic_message.push_str(&line.to_string());
    } else {
        panic_message.push_str(" no location information");
    }

    kprintln!("{}", panic_message);

    usr_thread_exit();
    loop {}
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
    usr_process_exit();
}
