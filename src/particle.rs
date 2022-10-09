use yew::{Html, html};
use rand::Rng;
use crate::math::{Vector2D};
use crate::mesh_field::SIZE;

#[derive(Copy, Clone)]
pub struct Particle {
    pub position: Vector2D,
    velocity: Vector2D,
    radius: f64,
}

impl Particle {
    pub fn new_random() -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            position: Vector2D::new(rng.gen::<f64>() * SIZE.x, rng.gen::<f64>() * SIZE.y),
            velocity: Vector2D::new(10.0 - rng.gen::<f64>() * 20.0, 10.0 - rng.gen::<f64>() * 20.0),
            radius: rng.gen::<f64>() * 3.0,
        }
    }

    fn border(&mut self) {
        //teleport
        if self.position.y < 0.0 {
            self.position.y = SIZE.y;
        }
        if self.position.x < 0.0 {
            self.position.x = SIZE.x;
        }
        if self.position.y > SIZE.y {
            self.position.y = 0.0;
        }
        if self.position.x > SIZE.x {
            self.position.x = 0.0;
        }
    }

    pub fn update_all(particles: &mut Vec<Particle>) {
        for x in particles {
            x.update();
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
        self.border();
    }

    pub fn render(&self) -> Html {
        let x = format!("{:.2}", self.position.x);
        let y = format!("{:.2}", self.position.y);
        let r = format!("{:.2}", self.radius);

        html! {
            <circle cx={x} cy={y} r={r} fill="white" />
        }
    }
}