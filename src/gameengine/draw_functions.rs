use crate::gameengine::color::Color;
use crate::gameengine::position::Position;
use crate::graphix::picturepainting::pictures::frame::Frame;

pub fn draw_circle(radius: u32, color: &Color, position: &Position, frame: &mut Frame) {
    let (cx, cy) = (position.get_x(), position.get_y());

    // quadratischer Bereich um den Kreis ablaufen
    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let x = cx + dx;
            let y = cy  + dy;

            // Abstand zum Mittelpunkt berechnen (Pythagoras)
            if dx * dx + dy * dy <= (radius * radius) as i32 {
                // Nur zeichnen, wenn (x, y) im Bild liegt
                if x >= 0 && y >= 0 && (x as u32) < frame.width && (y as u32) < frame.height {

                    frame.set_color_on_position(color, &Position::new(x, y));
                }
            }
        }
    }
}


pub fn draw_line(start: &Position, end: &Position, color: &Color, thickness: u32, frame: &mut Frame) {
    let (mut x0, mut y0) = (start.get_x(), start.get_y());
    let (x1, y1) = (end.get_x(), end.get_y());

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    while x0 != x1 || y0 != y1 {
        let line_pos = Position::new(x0, y0);
        draw_circle(thickness / 2, color, &line_pos, frame);

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }

    // Endpunkt nicht vergessen
    draw_circle(thickness / 2, color, end, frame);
}


fn draw_cross(color: &Color, position: &Position, frame: &mut Frame) {
    // Strich von oben nach unten
    for i in 0i32..11 {
        frame.set_color_on_position(color, &Position::new(position.get_x() + (i - 5), position.get_y() -1));
        frame.set_color_on_position(color, &Position::new(position.get_x() + (i - 5), position.get_y()));
        frame.set_color_on_position(color, &Position::new(position.get_x() + (i - 5), position.get_y() +1));
    }

    // Strich von links nach rechts
    for i in 0i32..11 {
        frame.set_color_on_position(color, &Position::new(position.get_x() -1 , position.get_y() + (i - 5)));
        frame.set_color_on_position(color, &Position::new(position.get_x(), position.get_y() + (i - 5)));
        frame.set_color_on_position(color, &Position::new(position.get_x() +1, position.get_y() + (i - 5)));
    }
}