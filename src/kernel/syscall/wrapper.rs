/*

use crate::kernel::syscall::user_api::usr_read_process_name;
use alloc::string::String;

pub fn get_process_name() -> String {
    let mut buffer = [0; 64];
    let len = usr_read_process_name(buffer.as_mut_ptr(), buffer.len());

    match core::str::from_utf8(&buffer[..len as usize]) {
        Ok(s) => String::from(s),
        Err(_) => String::from("<invalid utf8>"),
    }
}
*/
