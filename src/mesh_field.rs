use yew::{Component,Context,html,Html};
use gloo::timers::callback::Interval;

use crate::particle::Particle;
use crate::line::Line;
use crate::math::Vector2D;

pub const SIZE: Vector2D = Vector2D::new(1920.0,1080.0);

pub enum Msg {
    Tick,
}

pub struct MeshField {
    particles: Vec<Particle>,
    interval: Interval,
}

impl Component for MeshField {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let particles = (0..100)
            .map(|_| Particle::new_random())
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(50, move || link.send_message(Msg::Tick))
        };

        Self { 
            particles,
            interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                Particle::update_all(&mut self.particles);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);
        
        html! {
            <svg class="simulation-window" viewBox={view_box}>
                //{ for self.boids.iter().map(Boid::render) }
                { render_lines(self.particles.clone()) }
                { for self.particles.iter().map(Particle::render) }
            </svg>
        }
    }
}

fn render_lines(particles: Vec<Particle>) -> Html {
    let mut lines = vec![];

    let max_len = 200.0;

    for h in 0..particles.len() {
        for k in 0..particles.len() {
            if h != k {
                let mut line = Line::new(particles[h].position, particles[k].position, 100.0);

                if line.len() < max_len {
                    line.alpha = (max_len -line.len()) / max_len;
                    lines.push(line);
                }
            }
        }
    }

    html! {
        { for lines.iter().map(Line::render) }
    }
}
