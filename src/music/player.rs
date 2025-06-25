use crate::kernel::syscall::user_api;
use crate::music::note::Note;
use alloc::vec::Vec;
use core::{mem, slice};

pub fn play_notes(notes: &[Note]) {
    // Noten erst für Syscall umwandeln
    let (prepared_notes, len) = serialize_notes(&notes);

    // Syscall ausführen
    user_api::usr_play_song_with_notes(prepared_notes, len);

    // Speicher manuell rekonstruieren und freigeben:
    let buffer = unsafe { Vec::from_raw_parts(prepared_notes as *mut u8, len, len) };
    drop(buffer);
}

fn serialize_notes(notes: &[Note]) -> (*const u8, usize) {
    let len = notes.len() * size_of::<Note>();
    let mut buffer = Vec::with_capacity(len);

    unsafe {
        let src_ptr = notes.as_ptr() as *const u8;
        let src_slice = slice::from_raw_parts(src_ptr, len);
        buffer.extend_from_slice(src_slice);

        let ptr = buffer.as_ptr();
        // Speicher darf nicht freigegeben werden, also verhindern wir Drop:
        mem::forget(buffer);

        (ptr, len)
    }
}

pub unsafe fn deserialize_notes(ptr: *const u8, len: usize) -> Vec<Note> {
    let num_notes = len / size_of::<Note>();
    let slice = slice::from_raw_parts(ptr as *const Note, num_notes);
    slice.to_vec()
}
