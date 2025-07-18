use crate::gameengine::position::Position;
use core::any::Any;

/// Einfache Rechteckstruktur zur Kollision
#[derive(Debug)]
pub struct BoundingBox {
    pos: Position, // linke obere Ecke
    width: usize,
    height: usize,
}

impl BoundingBox {
    pub fn new(pos: &Position, width: usize, height: usize) -> BoundingBox {
        BoundingBox {
            pos: pos.clone(),
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let self_left = self.pos.get_x();
        let self_right = self.pos.get_x() + self.width as i32;
        let self_top = self.pos.get_y();
        let self_bottom = self.pos.get_y() + self.height as i32;

        let other_left = other.pos.get_x();
        let other_right = other.pos.get_x() + other.width as i32;
        let other_top = other.pos.get_y();
        let other_bottom = other.pos.get_y() + other.height as i32;

        // Scheiden sich die x Coordinatenlinien
        if self_right < other_left          // Self ist komplett links von other
        || other_right < self_left {        // Self ist komplett rechts von other
            return false; // X-Koordinaten sind komplett disjunkt
        }

        // Ab hier überschneiden sich die Boxen auf dem y-Intervall
        if self_bottom < other_top          // Self ist komplett über other
        || other_bottom < self_top {       // Self ist komplett unter other
            return false; // X-Coordinate scheidet zwar, aber Y-Koordinaten sind komplett disjunkt
        }

        // Hier schneiden sich sowohl die x-Intervalle als auch die y-Intervalle -> hit!
        return true;

    }
}

// TODO: Funktioniert bisher nur rechteckig
pub trait Collider {
    fn get_bounding_box(&self) -> BoundingBox;

    fn set_position(&mut self, position: &Position);

    fn get_position(&self) -> Position;

    fn move_position(&mut self, direction: &Position){
        let current = self.get_position();
        self.set_position(&(current + *direction));
    }

    fn test_collision(&self, other: &dyn Collider) -> bool {
        let bb1 = self.get_bounding_box();
        let bb2 = other.get_bounding_box();
        bb1.intersects(&bb2)
    }
    fn as_any(&self) -> &dyn Any; // Für Downcasting
}
