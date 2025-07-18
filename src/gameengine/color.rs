use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::kernel::syscall::user_api::usr_get_systime;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8, // Wenn alpha < 127 dann ist die Farbe durchsichtig
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        return Color {
            red,
            green,
            blue,
            alpha,
        };
    }

    pub fn is_transparent(&self) -> bool {
        return self.alpha < 127;
    }

    pub fn random_color() -> Color {
        let mut small_rng = SmallRng::seed_from_u64(usr_get_systime());
        // Alle Farben zufällig aber mit hohem Alpha
        Color::new(small_rng.gen(), small_rng.gen(), small_rng.gen(), 255)
    }
}

// * * Einige Farb-Konstanten * * //
pub const RED: Color = Color {
    red: 0xFF,
    green: 0,
    blue: 0,
    alpha: 0xFF,
};
pub const GREEN: Color = Color {
    red: 0,
    green: 0xFF,
    blue: 0,
    alpha: 0xFF,
};
pub const BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 0xFF,
    alpha: 0xFF,
};
pub const WHITE: Color = Color {
    red: 0xFF,
    green: 0xFF,
    blue: 0xFF,
    alpha: 0xFF,
};
pub const BLACK: Color = Color {
    red: 0x00,
    green: 0x00,
    blue: 0x00,
    alpha: 0xFF,
};
pub const TRANSPARENT: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 0,
};

pub const GRAY: Color = Color {
    red: 0x80,
    green: 0x80,
    blue: 0x80,
    alpha: 0xFF,
};
pub const LIGHT_GRAY: Color = Color {
    red: 0xC0,
    green: 0xC0,
    blue: 0xC0,
    alpha: 0xFF,
};
pub const DARK_GRAY: Color = Color {
    red: 0x40,
    green: 0x40,
    blue: 0x40,
    alpha: 0xFF,
};
pub const DARKER_GRAY: Color = Color {
    red: 0x1A,
    green: 0x1A,
    blue: 0x1A,
    alpha: 0xFF,
};

pub const YELLOW: Color = Color {
    red: 0xFF,
    green: 0xFF,
    blue: 0x00,
    alpha: 0xFF,
};
pub const CYAN: Color = Color {
    red: 0x00,
    green: 0xFF,
    blue: 0xFF,
    alpha: 0xFF,
};
pub const MAGENTA: Color = Color {
    red: 0xFF,
    green: 0x00,
    blue: 0xFF,
    alpha: 0xFF,
};

pub const ORANGE: Color = Color {
    red: 0xFF,
    green: 0xA5,
    blue: 0x00,
    alpha: 0xFF,
};
pub const PURPLE: Color = Color {
    red: 0x80,
    green: 0x00,
    blue: 0x80,
    alpha: 0xFF,
};
pub const BROWN: Color = Color {
    red: 0xA5,
    green: 0x2A,
    blue: 0x2A,
    alpha: 0xFF,
};
pub const PINK: Color = Color {
    red: 0xFF,
    green: 0xC0,
    blue: 0xCB,
    alpha: 0xFF,
};
