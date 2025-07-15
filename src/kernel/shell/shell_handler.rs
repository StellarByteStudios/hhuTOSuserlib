use crate::kernel::syscall::user_api::{
    usr_activate_shell, usr_clear_screen, usr_deactivate_shell, usr_get_screen_height,
    usr_get_screen_width,
};

// Einfach nur Syscalls aufrufen
// Lasst die Shell wieder Zeichen lesen
pub fn activate_shell() {
    usr_activate_shell();
}

// Lasst die Shell keine Zeichen mehr lesen
pub fn deactivate_shell() {
    usr_deactivate_shell();
}

// Holen der Bildschirmgröße
pub fn get_screen_size() -> (u64, u64) {
    // Größen über Syscalls holen
    let widht = usr_get_screen_width();
    let height = usr_get_screen_height();
    return (widht, height);
}

pub fn clear_screen(rainbow: bool) {
    // Wenn rainbow true -> color_code = 1
    if rainbow {
        usr_clear_screen(1);
        return;
    }

    usr_clear_screen(0)
}
