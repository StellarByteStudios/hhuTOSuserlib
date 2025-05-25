use crate::kernel::syscall::user_api::usr_read_process_name;

use alloc::string::String;

pub fn get_process_name() -> String {
    let mut app_name_buff: [u8; 64] = [0; 64];
    let ptr = app_name_buff.as_mut_ptr();
    let name_len = usr_read_process_name(ptr, 64);

    let string = String::new();
    for i in 0..name_len {
        unsafe {
            let byte = *app_name_buff.add(i);
            string.push(byte);
        }
    }
    
    string
}