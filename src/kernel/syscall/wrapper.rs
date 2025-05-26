use crate::kernel::syscall::user_api::usr_read_process_name;
use crate::kernel::syscall::user_api::usr_get_app_matching_name;

use alloc::string::String;

pub fn get_process_name() -> String {
    let mut app_name_buff: [u8; 64] = [0; 64];
    let ptr = app_name_buff.as_mut_ptr();
    let name_len = usr_read_process_name(ptr, 64);

    let mut string = String::new();
    for i in 0..name_len as usize {        
        let byte = app_name_buff[i];
        string.push(byte as char);
    }
    
    string
}

pub fn get_app_matching_name(search_name: String) -> String {
    let search_name_ptr = search_name.as_ptr();
    let search_name_len = search_name.len();

    let mut app_name_buff: [u8; 64] = [0; 64];
    let ptr = app_name_buff.as_mut_ptr();
    let name_len = usr_get_app_matching_name(search_name_ptr, search_name_len, ptr, 64);

    let mut string = String::new();
    for i in 0..name_len as usize {        
        let byte = app_name_buff[i];
        string.push(byte as char);
    }
    
    string
}