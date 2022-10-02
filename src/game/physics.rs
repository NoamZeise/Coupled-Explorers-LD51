use geometry::*;
use super::Colour;

#[derive(PartialEq, Clone, Copy)]
pub enum LastUpdate {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub struct PhysRect {
    pub rect: Rect,
    pub s : Vec2,
    pub v : Vec2,
    pub a : Vec2,
    pub max_v : Vec2,
    pub prev_s: Vec2,
    pub last_update: LastUpdate,
    pub weight: f64,
    pub x_collision: bool,
    pub y_collision: bool,
    pub friction : f64,
    pub colour: Colour,
}

impl PhysRect {
    pub fn new(rect: Rect, max_v: Vec2) -> Self {
        PhysRect {
            rect,
            s: Vec2::new(rect.x, rect.y),
            v: Vec2::new(0.0, 0.0),
            a: Vec2::new(0.0, 0.0),
            max_v,
            prev_s: Vec2::new(rect.x, rect.y),
            last_update: LastUpdate::X,
            weight: 1.0,
            x_collision: false,
            y_collision: false,
            friction : 1.0,
            colour: Colour::black(),
        }
    }
    pub fn new_from_rect(rect: Rect) -> Self {
        PhysRect {
            rect,
            s: Vec2::new(rect.x, rect.y),
            v: Vec2::new(0.0, 0.0),
            a: Vec2::new(0.0, 0.0),
            max_v: Vec2::new(0.0, 0.0),
            prev_s: Vec2::new(rect.x, rect.y),
            last_update: LastUpdate::X,
            weight: 1.0,
            x_collision: false,
            y_collision: false,
            friction: 1.0,
            colour: Colour::black(),
        }
    }

    pub fn update_x(&mut self, time: &f64) {
        if !self.x_collision {
            self.v.x *= self.friction;
        }
        self.x_collision = false;
        self.prev_s.x = self.s.x;
        self.last_update = LastUpdate::X;
        self.v.x += self.a.x * time;
        self.a.x = 0.0;
        limit(&mut self.v.x, self.max_v.x);
        self.s.x += self.v.x * time;
        self.rect.x = self.s.x;
    }

    pub fn update_y(&mut self, time: &f64) {
        self.y_collision = false;
        self.prev_s.y = self.s.y;
        self.last_update = LastUpdate::Y;
        self.v.y += self.a.y * time;
        limit(&mut self.v.y, self.max_v.y);
        self.s.y += self.v.y * time;
        self.rect.y = self.s.y;
    }

    pub fn get_pixel_correct_rect(&self) -> Rect {
        Rect::new(
            self.rect.x.round(),
            self.rect.y.round(),
            self.rect.w,
            self.rect.h
        )
    }
}

fn limit(n: &mut f64, max: f64) {
    if n.abs() > max && max != 0.0 {
        *n = n.signum() * max;
    }
}

pub trait Phys {
    fn pr(&mut self) -> &mut PhysRect;
    fn pr_im(&self) -> &PhysRect;
    fn pre_physics(&mut self) { } 
    fn phys_x(&mut self, time: &f64) {
        self.pr().update_x(time);
    }
    fn phys_y(&mut self, time: &f64) {
        self.pr().update_y(time);
    }
    fn collision(&mut self, other: &PhysRect) {
        match self.pr().last_update {
            LastUpdate::X => {
                self.pr().x_collision = true;
                self.pr().s.x = resolve_x(self.pr().prev_s.x, self.pr().rect, &other.rect);
                self.pr().rect.x = self.pr().s.x;
                self.pr().v.x = momentum(
                    self.pr().v.x, self.pr().weight, other.v.x, other.weight
                )
            }
            LastUpdate::Y => {
                self.pr().y_collision = true;
                self.pr().s.y = resolve_y(self.pr().prev_s.y, self.pr().rect, &other.rect);
                self.pr().rect.y = self.pr().s.y;
                self.pr().v.y = 0.0;/*momentum(
                    self.pr().v.y, self.pr().weight, other.v.y, other.weight
                )*/
            }
        }
    }
    fn post_physics(&mut self) { }
}


pub fn collision_update<A: Phys + ?Sized, B: Phys + ?Sized>(a: &mut A, b: &mut B) {
    if a.pr().rect.colliding(&b.pr().rect) {
        let a_phys = *a.pr();
        a.collision(b.pr());
        b.collision(&a_phys);
    }
}

fn momentum(u1: f64, m1: f64, u2: f64, m2: f64) -> f64 {
    if m2 == 0.0 { return 0.0; }
    let tm = m1 + m2;
    if tm == 0.0 { return 0.0; }
    ((m1 - m2)/tm)*u1 + ((m2*2.0)/tm)*u2
}

fn resolve_x(v: f64, a: Rect, b: &Rect) -> f64 {
    let mut a = a;
    let dir = (v - a.x).signum();
    while (v - a.x).signum() != dir {
        a.x += dir/2.0;
        if !a.colliding(b) {
            return a.x;
        }
    }
    v
}
fn resolve_y(v: f64, a: Rect, b: &Rect) -> f64 {
    let mut a = a;
    let dir = (v - a.y).signum();
    while (v - a.y).signum() == dir {
        a.y += dir/5.0;
        if !a.colliding(b) {
            return a.y;
        }
    }
    v
}
