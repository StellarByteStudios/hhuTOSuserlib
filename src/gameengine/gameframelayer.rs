use crate::{
    gameengine::{
        color::{Color, TRANSPARENT},
        draw_functions,
        position::Position,
    },
    graphix::picturepainting::{paint::draw_picture, pictures::frame::Frame},
};

#[derive(Debug)]
pub struct GameFrameLayer {
    field_size: (usize, usize),
    frame: Frame,
}

impl GameFrameLayer {
    pub fn new(width: usize, height: usize) -> GameFrameLayer {
        let field_size = (width, height);

        let boardframe: Frame = Frame::new(width as u32, height as u32);

        return GameFrameLayer {
            field_size: field_size,
            frame: boardframe,
        };
    }

    pub fn get_field_size(&self) -> (usize, usize) {
        return self.field_size;
    }

    pub fn paint(&self, pos: &Position) {
        draw_picture(pos.get_x() as usize, pos.get_y() as usize, &self.frame)
    }

    pub fn paint_layers(game_frame_layers: &[GameFrameLayer], pos: &Position) {
        // Gameframes zusammen mergen
        let mut frame_buffer = Frame::new(
            game_frame_layers[0].field_size.0 as u32,
            game_frame_layers[0].field_size.1 as u32,
        );

        // Einzelne Layer zusammen packen
        for game_frame_layer in game_frame_layers.iter() {
            frame_buffer.copy_from_frame(&game_frame_layer.frame)
        }

        // Als Block ausgeben
        frame_buffer.print_frame_on_pos(pos);
    }

    pub fn draw_sprite_on_position(&mut self, position: &Position, sprite: &Frame) -> bool {
        // Border Checken
        if (position.get_x() + sprite.width as i32) as usize >= self.field_size.0 {
            // Zu weit rechts außen
            return false;
        }
        if (position.get_y() + sprite.height as i32) as usize >= self.field_size.1 {
            // zu weit unten
            return false;
        }

        // Sprite in GameFrame packen
        for x in 0..sprite.width {
            for y in 0..sprite.height {
                // Farbe aus Sprite holen
                let color = sprite.get_color_on_position(&Position::new_u32(x, y));

                // Position im Gameframe berechnen
                let pixel_pos =
                    Position::new(position.get_x() + x as i32, position.get_y() + y as i32);

                // Pixel übertragen
                self.frame.set_color_on_position(&color, &pixel_pos);
            }
        }

        return true;
    }

    pub fn delete_sprite_on_position(&mut self, position: &Position, sprite: &Frame) -> bool {
        // Border Checken
        if (position.get_x() + sprite.width as i32) as usize >= self.field_size.0 {
            // Zu weit rechts außen
            return false;
        }
        if (position.get_y() + sprite.height as i32) as usize >= self.field_size.1 {
            // zu weit unten
            return false;
        }

        // Sprite in GameFrame packen
        for x in 0..sprite.width {
            for y in 0..sprite.height {
                // Farbe aus Sprite holen
                let color = sprite.get_color_on_position(&Position::new_u32(x, y));

                // Position im Gameframe berechnen
                let pixel_pos =
                    Position::new(position.get_x() + x as i32, position.get_y() + y as i32);

                // Wenn der Pixel nicht durchsichtig war im Sprite, dann jetzt durchsichtig machen
                if !color.is_transparent() {
                    self.frame.set_color_on_position(&TRANSPARENT, &pixel_pos);
                }
            }
        }

        return true;
    }
}

// * * Impl für die Funktionen aus draw_functions * * //
impl GameFrameLayer {
    pub fn draw_circle(&mut self, center: &Position, radius: u32, color: &Color) {
        draw_functions::draw_circle(radius, color, center, &mut self.frame);
    }

    pub fn draw_line(&mut self, start: &Position, end: &Position, color: &Color, thickness: u32) {
        draw_functions::draw_line(start, end, color, thickness, &mut self.frame);
    }

    pub fn draw_cross(&mut self, center: &Position, color: &Color) {
        draw_functions::draw_cross(color, center, &mut self.frame);
    }

    pub fn draw_frame_border(&mut self, color: &Color) {
        draw_functions::set_border(color, &mut self.frame);
    }

    pub fn fill_frame(&mut self, color: &Color) {
        self.frame.fill_frame(color);
    }
}
