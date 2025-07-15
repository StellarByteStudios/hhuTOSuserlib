use crate::gameengine::position::Position;
use crate::graphix::picturepainting::pictures::frame::Frame;

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

    pub fn draw_on_position(&mut self, position: &Position, sprite: &Frame) -> bool {
        // Border Checken
        if (position.get_x() + sprite.width as i32) as usize >= self.field_size.0 {
            // Zu weit rechts außen
            return false;
        }
        if (position.get_y() + sprite.height as i32) as usize>= self.field_size.1 {
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
}
