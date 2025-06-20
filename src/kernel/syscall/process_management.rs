use crate::kernel::syscall::user_api::{
    usr_kill_process, usr_print_running_thread, usr_process_exit, usr_read_process_name,
    usr_thread_exit,
};
use alloc::string::String;

pub fn get_process_name() -> String {
    let mut buffer = [0; 64];
    let len = usr_read_process_name(buffer.as_mut_ptr(), buffer.len());

    match core::str::from_utf8(&buffer[..len as usize]) {
        Ok(s) => String::from(s),
        Err(_) => String::from("<invalid utf8>"),
    }
}

pub fn print_running_threads() {
    usr_print_running_thread();
}

pub fn exit() {
    usr_thread_exit()
}

pub fn exit_process() {
    usr_process_exit();
}

pub fn kill_process(pid: usize) {
    usr_kill_process(pid);
}
