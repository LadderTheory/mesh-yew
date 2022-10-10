use gloo::utils::document;
use yew::{Component,Context,html,Html};
use gloo::timers::callback::Interval;
use web_sys::{Document, Element};

use crate::particle::Particle;
use crate::line::Line;
use crate::math::Vector2D;

//pub const SIZE: Vector2D = Vector2D::new(1920.0,1080.0);

pub enum Msg {
    Tick,
}

pub struct MeshField {
    particles: Vec<Particle>,
    lines: Vec<Line>,
    interval: Interval,
    max_line_len: f64,
    size: Vector2D,
}

impl Component for MeshField {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let size = get_size();

        let particles = (0..20)
            .map(|_| Particle::new_random("red".to_string(), size))
            .collect();

        let interval = {
            let link = ctx.link().clone();
            Interval::new(50, move || link.send_message(Msg::Tick))
        };

        Self { 
            particles,
            lines: vec![],
            interval,
            max_line_len: 200.0,
            size,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.size = get_size();
                self.update_particles();
                self.update_lines();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", self.size.x, self.size.y);

        let mut middles = vec![];

        for i in 0..self.lines.len() {
            middles.push(Particle::new(self.lines[i].middle(), Vector2D::zero(), self.particles[0].radius, "white".to_string(), self.lines[i].alpha, self.size));
        }
        
        html! {
            <svg class="simulation-window" viewBox={view_box}>
                { for self.particles.iter().map(Particle::render) }
                { for self.lines.iter().map(|x| Line::render(x, "white".to_string())) }
                //{ for middles.iter().map(Particle::render) }
                //{ for self.draw_lines(middles).iter().map(|x| Line::render(x, "red".to_string())) }
            </svg>
        }
    }
}

impl MeshField {
    fn update_lines(&mut self) {
        //self.lines = self.draw_lines(self.particles.clone());
        let new_lines = self.draw_lines(self.particles.clone());
        for h in 0..new_lines.len() {
            self.lines.push(new_lines[h].clone());
        }
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
                        line.alpha = ((self.max_line_len - line_len) / self.max_line_len) ;
                        lines.push(line)
                    }
                }
                
            }
        }

        lines
    }

    fn update_particles(&mut self) {
        for h in 0..self.particles.len() {
            self.particles[h].field_size = self.size;
            self.particles[h].update();
        }
    }
}

impl Line {
    pub fn render(&self, color: String) -> Html {
        let style = format!("stroke-opacity: {:.2}", self.alpha);

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

        let style = format!("opacity: {:.2}", self.alpha);

        html! {
            <circle cx={x} cy={y} r={r} fill={self.color.clone()} {style} />
        }
    }
}

fn str(x: f64) -> String {
    format!("{:.2}", x)
}

fn get_size() -> Vector2D {
    let body = document().body().unwrap();

    Vector2D::new(body.client_width() as f64, body.client_height() as f64)
}