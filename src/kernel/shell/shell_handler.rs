use crate::kernel::syscall::user_api::{usr_activate_shell, usr_deactivate_shell};

// Einfach nur Syscalls aufrufen
// Lasst die Shell wieder Zeichen lesen
pub fn activate_shell() {
    usr_activate_shell();
}

// Lasst die Shell keine Zeichen mehr lesen
pub fn deactivate_shell() {
    usr_deactivate_shell();
}
