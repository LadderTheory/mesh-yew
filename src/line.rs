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

    pub fn len(&self) -> f64 {
        Vector2D::distance(self.p1, self.p2)
    }

    pub fn render(&self) -> Html {
        //let positions = format!{"x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"", self.p1.x, self.p1.y, self.p2.x, self.p2.y};

        let style = format!("stroke-opacity: {:.2}", self.alpha);

        html! {
            <line 
                x1={ str(self.p1.x) }
                y1={ str(self.p1.y) }
                x2={ str(self.p2.x) }
                y2={ str(self.p2.y) }
                stroke="red"
                { style }
            />
        }
    }
}

fn str(x: f64) -> String {
    format!("{:.2}", x)
}