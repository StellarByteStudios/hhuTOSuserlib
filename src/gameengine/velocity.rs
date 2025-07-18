use crate::gameengine::directions::Direction;
use crate::utility::mathadditions::math::sqrt;
use core::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
}

// * * Boiler Plate * * //
impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }

    pub fn new_zero() -> Velocity {
        Velocity { x: 0.0, y: 0.0 }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl PartialEq for Velocity {
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

// * * Grundrechenarten * * //
impl Add for Velocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Velocity {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Velocity {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Velocity {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Velocity {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Velocity {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, multiplicand: f32) -> Self {
        Velocity {
            x: self.x * multiplicand,
            y: self.y * multiplicand,
        }
    }
}

impl Mul<u32> for Velocity {
    type Output = Self;

    fn mul(self, multiplicand: u32) -> Self {
        Velocity {
            x: self.x * multiplicand as f32,
            y: self.y * multiplicand as f32,
        }
    }
}

impl Mul<usize> for Velocity {
    type Output = Self;

    fn mul(self, multiplicand: usize) -> Self {
        Velocity {
            x: self.x * multiplicand as f32,
            y: self.y * multiplicand as f32,
        }
    }
}

impl Div<f32> for Velocity {
    type Output = Velocity;

    fn div(self, denominator: f32) -> Velocity {
        Velocity {
            x: self.x / denominator,
            y: self.y / denominator,
        }
    }
}

impl Velocity {
    pub fn get_abs_speed(&self) -> f32 {
        return sqrt(self.x * self.x + self.y * self.y);
    }

    pub fn dot(&self, other: Velocity) -> f32 {
        return sqrt(self.x * other.x + self.y * other.y);
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    // Gibt eine normalisierte Version des Vektors zurück (Länge = 1)
    pub fn normalize(self) -> Self {
        let length = sqrt(self.x * self.x + self.y * self.y);
        if length != 0.0 {
            Velocity {
                x: self.x / length,
                y: self.y / length,
            }
        } else {
            self // Länge 0 bleibt unverändert
        }
    }
}

// * * Fancy Stuff * * //
impl Velocity {
    pub fn bounce_on(&mut self, border: Direction) {
        match border {
            Direction::Up | Direction::Down => self.y = -self.y,
            Direction::Left | Direction::Right => self.x = -self.x,
        }
    }
}
