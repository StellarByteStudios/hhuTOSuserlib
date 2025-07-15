use crate::gameengine::velocity::Velocity;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

// * * Boiler Plate * * //
impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    pub fn new_u32(x: u32, y: u32) -> Position {
        return Position::new(x as i32, y as i32);
    }

    pub fn new_zero() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        if self.x != other.y {
            return false;
        }

        if self.x != other.y {
            return false;
        }

        return true;
    }
}

// * * Fancy Stuff * * //
impl Position {
    pub fn shift(&mut self, vel: Velocity) {
        self.x += vel.get_x() as i32;
        self.y += vel.get_y() as i32;
    }
}
