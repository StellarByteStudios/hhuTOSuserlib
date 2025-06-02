use crate::kernel::syscall::user_api::usr_read_process_name;
use crate::kernel::syscall::user_api::usr_get_app_matching_name;

pub fn get_process_name(buffer: &mut [u8]) -> &str {
    let len = usr_read_process_name(buffer.as_mut_ptr(), buffer.len());

    match core::str::from_utf8(&buffer[..len as usize]) {
        Ok(s) => s,
        Err(_) => "<invalid utf8>",
    }   
}

pub fn get_app_matching_name<'a>(buffer: &'a mut [u8], search_name: &[u8], search_name_len: usize) -> &'a str {    
    let name_len = usr_get_app_matching_name(search_name.as_ptr(), search_name_len, buffer.as_mut_ptr(), 64);

    match core::str::from_utf8(&buffer[..name_len as usize]) {
        Ok(s) => s,
        Err(_) => "<invalid utf8>",
    } 
}