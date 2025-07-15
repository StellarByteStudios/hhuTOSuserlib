use crate::kernel::syscall::user_api::usr_getlastkey;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyEvent {
    NoEvent,
    KeyDown(u8),
}

const ASCII_LIMIT: usize = 128;
// Umwandlungsfunktion, welche einen KeyEvent aus einem usize erzeugt
impl From<usize> for KeyEvent {
    fn from(value: usize) -> Self {
        // Teste ob es ein valider Character ist
        if value > 0 && value < ASCII_LIMIT {
            // Character umwandeln
            return KeyEvent::KeyDown(value as u8);
        }
        // Kein neues Event
        return KeyEvent::NoEvent;
    }
}

impl From<u64> for KeyEvent {
    fn from(value: u64) -> Self {
        // Teste ob es ein valider Character ist
        if value > 0 && value < ASCII_LIMIT as u64 {
            // Character umwandeln
            return KeyEvent::KeyDown(value as u8);
        }
        // Kein neues Event
        return KeyEvent::NoEvent;
    }
}

impl From<KeyEvent> for usize {
    fn from(event: KeyEvent) -> usize {
        match event {
            KeyEvent::KeyDown(c) => c as usize,
            KeyEvent::NoEvent => 0,
        }
    }
}

impl KeyEvent {
    pub fn as_char(&self) -> char {
        match self {
            KeyEvent::KeyDown(c) => *c as char,
            KeyEvent::NoEvent => 0 as char,
        }
    }
}

pub fn get_new_key_event() -> KeyEvent {
    let key = usr_getlastkey();
    let keyevent = KeyEvent::from(key);
    return keyevent;
}

pub fn get_last_key() -> char {
    loop {
        // Event holen
        let key_event = get_new_key_event();

        // Gabs was neues?
        if key_event == KeyEvent::NoEvent {
            continue;
        }

        // Key auspacken
        return key_event.as_char();
    }
}
