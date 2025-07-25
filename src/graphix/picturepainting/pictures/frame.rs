use alloc::{vec, vec::Vec};
use core::fmt;

use crate::{
    gameengine::{
        color::{Color, TRANSPARENT},
        position::Position,
    },
    graphix::picturepainting::paint::draw_picture,
};

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub data: Vec<u8>,
}

impl Frame {
    // Konstruktor für neuen Transparenten Frame
    pub fn new(width: u32, height: u32) -> Self {
        let mut frame: Frame = Frame {
            width: width,
            height: height,
            bpp: 4,
            data: vec![0u8; (width * height * 4) as usize],
        };

        frame.data.fill(0x00); // Komplett Transparent füllen

        return frame;
    }

    // Übersetzung von Position zu index
    fn pos_to_index(&self, pos: &Position) -> usize {
        return self.xy_to_index(pos.get_x() as u32, pos.get_y() as u32) as usize;
    }

    fn xy_to_index(&self, x: u32, y: u32) -> u32 {
        return y * self.width + x;
    }

    // Malt den Pixel an einem bestimmten Index
    fn set_color_on_pixel_index(&mut self, color: &Color, index: usize) {
        let i = index * 4;

        self.data[i] = color.red;
        self.data[i + 1] = color.green;
        self.data[i + 2] = color.blue;
        self.data[i + 3] = color.alpha;
    }

    // Holt die Farbe aus einem bestimmten Index
    fn get_color_on_pixel_index(&self, index: usize) -> Color {
        let i = index * 4;

        let mut color: Color = TRANSPARENT;

        color.red = self.data[i];
        color.green = self.data[i + 1];
        color.blue = self.data[i + 2];
        color.alpha = self.data[i + 3];

        return color;
    }

    // Checkt ob eine Anfrage innerhalb des Frames ist
    fn check_frame_border(&self, pos: &Position) -> bool {
        return pos.get_x() >= self.width as i32 || pos.get_y() >= self.height as i32;
    }

    pub fn fill_frame(&mut self, color: &Color) {
        for i in 0..self.width * self.height - 1 {
            self.set_color_on_pixel_index(color, i as usize);
        }
    }

    pub fn copy_from_frame(&mut self, frame: &Frame) {
        for i in 0..self.width * self.height {
            // Holt den Pixel aus dem anderen Frame
            let color = frame.get_color_on_pixel_index(i as usize);

            // Wenn die Farbe nicht transparent ist, wird sie kopiert
            if !color.is_transparent() {
                self.set_color_on_pixel_index(&color, i as usize);
            }
        }
    }

    pub fn print_frame_on_pos(&self, pos: &Position) {
        draw_picture(pos.get_x() as usize, pos.get_y() as usize, self);
    }

    pub fn set_color_on_position(&mut self, color: &Color, pos: &Position) {
        // Border Checken
        if self.check_frame_border(pos) {
            // Aus dem Frame rausgelaufen
            return;
        }

        // Index berechnen
        let index = self.pos_to_index(pos);

        // Pixel setzen
        self.set_color_on_pixel_index(color, index);
    }

    pub fn get_color_on_position(&self, pos: &Position) -> Color {
        // Border Checken
        if self.check_frame_border(pos) {
            // Aus dem Frame rausgelaufen
            return TRANSPARENT;
        }

        // Index berechnen
        let index = self.pos_to_index(pos);

        // Pixel holen
        return self.get_color_on_pixel_index(index);
    }
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("bpp", &self.bpp)
            .field("data", &format_args!("<{} bytes>", self.data.len()))
            .finish()
    }
}
