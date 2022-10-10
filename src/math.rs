use std::ops::{AddAssign,MulAssign,Sub,Add,Div};

#[derive(Copy,Clone,PartialEq,Default)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(x: Vector2D, y: Vector2D) -> f64 {
        (x - y).hypotenuse()
    }

    pub fn hypotenuse(&mut self) -> f64 {
        f64::sqrt((self.x*self.x) + (self.y*self.y))
    }

    pub const fn zero() -> Vector2D {
        Vector2D::new(0.0, 0.0)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl MulAssign for Vector2D {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Div for Vector2D {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}