use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::slice_from_raw_parts;

// Konstanten für den Viruellen Adressraum
pub const USER_SPACE_START: usize = 0x100_0000_0000; // 1TiB
pub const USER_SPACE_CODE_START: usize = USER_SPACE_START;
pub const USER_SPACE_ENV_START: usize = USER_SPACE_CODE_START + 0x4000_0000;
pub const USER_SPACE_ARG_START: usize = USER_SPACE_ENV_START;

pub(crate) const ARGC_PTR: *const usize = USER_SPACE_ARG_START as *const usize;
pub(crate) const ARGV_PTR: *const *const u8 =
    (USER_SPACE_ARG_START + size_of::<*const usize>()) as *const *const u8;

pub struct Args {
    index: usize,
}

// Gibt die Argumente als Iterator zurück
pub fn args() -> Args {
    Args::new()
}

impl Args {
    fn new() -> Self {
        Args { index: 0 }
    }
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // Wie vieltes Argument sind wir grade?
            let argc = *ARGC_PTR;

            // Sind wir noch argumente übrig? Sonst leeren Optional
            if self.index >= argc {
                return None;
            }

            // Pointer auf das i.te element der args
            let arg = *ARGV_PTR.add(self.index);
            // Wie lang ist dieses Argument?
            let len = strlen(arg);
            // Iterator weiter schieben
            self.index += 1;

            // Schaut nach ob man das nächste Argument zu nem String Casten kann
            // Falls ja, wird dieser Zurück gegeben im Iterator, sonst Fehler
            CStr::from_bytes_with_nul(slice_from_raw_parts(arg, len + 1).as_ref()?)
                .map(|cstr| {
                    cstr.to_str()
                        .expect("Invalid UTF-8 in argument")
                        .to_string()
                })
                .ok()
        }
    }
}

/* Bestimmen der Länge eines null-terminierten String */
pub fn strlen(str: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *str.add(len) != 0 {
            len += 1;
        }
    }

    return len;
}

// Gibt die Argumente als einen Vector zurück
pub fn args_as_vec() -> Vec<String> {
    let args = args();
    let mut vec = Vec::new();
    for arg in args {
        vec.push(arg);
    }
    vec
}

pub fn args_len() -> usize {
    unsafe {
        return *ARGC_PTR;
    }
}
