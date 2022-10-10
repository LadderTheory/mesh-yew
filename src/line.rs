use yew::{Html,html};
use crate::math::{Vector2D};

pub struct Line {
    p1: Vector2D,
    p2: Vector2D,
    pub alpha: f64,
}

impl Line {
    pub fn new(p1: Vector2D, p2: Vector2D, alpha: f64) -> Self {
        Self {
            p1,
            p2,
            alpha,
        }
    }

    pub fn get_points(&self) -> (Vector2D, Vector2D) {
        (self.p1, self.p2)
    }

    pub fn len(&self) -> f64 {
        Vector2D::distance(self.p1, self.p2)
    }

    pub fn middle(&self) -> Vector2D {
        self.sum() / Vector2D::new(2.0, 2.0)
    }

    pub fn sum(&self) -> Vector2D {
        self.p1 + self.p2
    }
}