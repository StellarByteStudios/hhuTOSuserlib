use crate::kernel::allocator::allocator::init;
use crate::kernel::runtime::environment;
use crate::kernel::syscall::user_api::{
    usr_get_pid, usr_panic_print, usr_process_exit, usr_thread_exit,
};
use core::panic::PanicInfo;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MB heap size

unsafe extern "C" {
    fn main(argc: isize, argv: *const *const u8) -> isize;
}

#[cfg(feature = "lib-panic-handler")] // Defaultfeature für Kernel deaktiviert -> panic Handler Dopplung
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // location der panic herausfiltern
    let (file_ptr, file_len, line) = if let Some(loc) = info.location() {
        let file = loc.file();
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
