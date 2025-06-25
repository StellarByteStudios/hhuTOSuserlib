use alloc::vec::Vec;

use super::pictures::blinking::{blinking0, blinking1, blinking2, blinking3, blinking4};
use crate::{
    graphix::picturepainting::{
        paint::draw_picture,
        pictures::charmander::{charmander0, charmander1, charmander2, charmander3, charmander4},
    },
    utility::delay::delay,
};

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub data: Vec<u8>,
}

pub fn animate_blink(x: u32, y: u32) {
    // Bilder laden
    let frames: [Frame; 5] = [
        Frame {
            width: blinking0::WIDTH,
            height: blinking0::HEIGHT,
            bpp: blinking0::BPP,
            data: blinking0::DATA.to_vec(),
        },
        Frame {
            width: blinking1::WIDTH,
            height: blinking1::HEIGHT,
            bpp: blinking1::BPP,
            data: blinking1::DATA.to_vec(),
        },
        Frame {
            width: blinking2::WIDTH,
            height: blinking2::HEIGHT,
            bpp: blinking2::BPP,
            data: blinking2::DATA.to_vec(),
        },
        Frame {
            width: blinking3::WIDTH,
            height: blinking3::HEIGHT,
            bpp: blinking3::BPP,
            data: blinking3::DATA.to_vec(),
        },
        Frame {
            width: blinking4::WIDTH,
            height: blinking4::HEIGHT,
            bpp: blinking4::BPP,
            data: blinking4::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    gprintln!("animating Blinking");
    loop {
        for i in 0..frames.len() {
            draw_picture(x as usize, y as usize, &frames[i]);

            delay(25);
        }
    }
}

pub fn animate_charmander(x: u32, y: u32) {
    // Bilder laden
    let frames: [Frame; 5] = [
        Frame {
            width: charmander0::WIDTH,
            height: charmander0::HEIGHT,
            bpp: charmander0::BPP,
            data: charmander0::DATA.to_vec(),
        },
        Frame {
            width: charmander1::WIDTH,
            height: charmander1::HEIGHT,
            bpp: charmander1::BPP,
            data: charmander1::DATA.to_vec(),
        },
        Frame {
            width: charmander2::WIDTH,
            height: charmander2::HEIGHT,
            bpp: charmander2::BPP,
            data: charmander2::DATA.to_vec(),
        },
        Frame {
            width: charmander3::WIDTH,
            height: charmander3::HEIGHT,
            bpp: charmander3::BPP,
            data: charmander3::DATA.to_vec(),
        },
        Frame {
            width: charmander4::WIDTH,
            height: charmander4::HEIGHT,
            bpp: charmander4::BPP,
            data: charmander4::DATA.to_vec(),
        },
    ];

    // Frames nacheinander zeichnen
    //gprintln!("animating Charmander");
    loop {
        for i in 0..frames.len() {
            draw_picture(x as usize, y as usize, &frames[i]);

            delay(10);
        }
    }
}
