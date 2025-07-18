use crate::gameengine::collision::collisiontrait::{BoundingBox, Collider};
use crate::gameengine::position::Position;
use core::any::Any;

pub struct Circle {
    center: Position,
    radius: usize,
}

impl Circle {
    pub fn new(center: Position, radius: usize) -> Circle {
        Circle { center, radius }
    }
}

impl Collider for Circle {
    // TODO: Gibt aktuell nur ein Recteck zurück zurück
    fn get_bounding_box(&self) -> BoundingBox {
        let up_left_corner = Position::new(
            self.center.get_x() - self.radius as i32,
            self.center.get_y() - self.radius as i32,
        );

        return BoundingBox::new(&up_left_corner, self.radius * 2, self.radius * 2);
    }

    fn set_position(&mut self, center: &Position) {
        self.center = center.clone();
    }

    fn get_position(&self) -> Position {
        self.center.clone()
    }

    fn test_collision(&self, other: &dyn Collider) -> bool {
        // Fall bei zwei Kreisen, die aufeinander Treffen
        if let Some(other_circle) = other.as_any().downcast_ref::<Circle>() {
            let dx = self.center.get_x() - other_circle.center.get_x();
            let dy = self.center.get_y() - other_circle.center.get_y();
            let distance_sq = dx * dx + dy * dy;
            let radius_sum = self.radius + other_circle.radius;
            return distance_sq as usize <= radius_sum * radius_sum;
        }

        // Allgemeiner Fall
        let bb1 = self.get_bounding_box();
        let bb2 = other.get_bounding_box();
        bb1.intersects(&bb2)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
