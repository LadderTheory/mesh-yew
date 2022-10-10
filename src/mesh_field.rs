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
    lines: Vec<Line>,
    interval: Interval,
    max_line_len: f64,
}

impl Component for MeshField {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let particles = (0..50)
            .map(|_| Particle::new_random("red".to_string()))
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(50, move || link.send_message(Msg::Tick))
        };

        let lines = vec![];

        let max_line_len = 200.0;

        Self { 
            particles,
            lines,
            interval,
            max_line_len,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.update_particles();
                self.update_lines();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", SIZE.x, SIZE.y);

        let mut middles = vec![];

        for i in 0..self.lines.len() {
            middles.push(Particle::new(self.lines[i].middle(), Vector2D::zero(), self.particles[0].radius, "red".to_string()));
        }
        
        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.lines.iter().map(|x| Line::render(x, "white".to_string())) }
                //{ for self.particles.iter().map(Particle::render) }
                //{ for middles.iter().map(Particle::render) }
                { for self.draw_lines(middles).iter().map(|x| Line::render(x, "red".to_string())) }
            </svg>
        }
    }
}

impl MeshField {
    fn update_lines(&mut self) {
        self.lines = self.draw_lines(self.particles.clone());
    }

    fn draw_lines(&self, particles: Vec<Particle>) -> Vec<Line> {
        let mut lines = vec![];
        let l = particles.len();

        for h in 0..l {
            for k in 0..l {
                if h != k {
                    let ph = &particles[h];
                    let pk = &particles[k];
                    let mut line = Line::new(ph.position, pk.position, 100.0);

                    let line_len = line.len();

                    if line_len < self.max_line_len {
                        line.alpha = (self.max_line_len - line_len) / self.max_line_len;
                        lines.push(line)
                    }
                }
                
            }
        }

        lines
    }

    fn update_particles(&mut self) {
        for h in 0..self.particles.len() {
            self.particles[h].update();
        }
    }
}

impl Line {
    pub fn render(&self, color: String) -> Html {let style = format!("stroke-opacity: {:.2}", self.alpha);

        let (p1,p2) = self.get_points();

        html! {
            <line 
                x1={ str(p1.x) }
                y1={ str(p1.y) }
                x2={ str(p2.x) }
                y2={ str(p2.y) }
                stroke={ color }
                { style }
            />
        }
    }
}

impl Particle {
    pub fn render(&self) -> Html {
        let x = format!("{:.2}", self.position.x);
        let y = format!("{:.2}", self.position.y);
        let r = format!("{:.2}", self.radius);

        html! {
            <circle cx={x} cy={y} r={r} fill={self.color.clone()} />
        }
    }
}

fn str(x: f64) -> String {
    format!("{:.2}", x)
}