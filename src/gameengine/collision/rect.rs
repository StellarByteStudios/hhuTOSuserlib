use crate::gameengine::collision::collisiontrait::{BoundingBox, Collider};
use crate::gameengine::position::Position;
use core::any::Any;

pub struct Rect {
    pos: Position,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(pos: Position, width: usize, height: usize) -> Rect {
        Rect{ pos, width, height }
    }
}

impl Collider for Rect {
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox::new(&self.pos, self.width, self.height)
    }

    fn get_position(&self) -> Position {
        self.pos.clone()
    }

    fn set_position(&mut self, position: &Position) {
        self.pos = position.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
