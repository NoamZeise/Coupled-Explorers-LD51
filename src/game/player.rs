use super::helper::*;
use super::physics::*;

use sdl2::mixer::Music;

//const ACC: Vec2 = Vec2 { x: 1000.0, y: 700.0 };
//const MAX_V: Vec2 = Vec2 { x: 150.0, y: 600.0 };
//const START_V : Vec2 = Vec2 { x: 0.0, y: -240.0  };

pub struct Player {
    pub go: GameObject,
    pr: PhysRect,
    acc: Vec2,
    jump: f64,
    frict: f64,
    jumped: bool,
}

impl  Player {
    pub fn new(tex: Texture,
               acc: Vec2,
               jump: f64,
               frict: f64,
               max_v: Vec2,
               weight: f64,
    ) -> Self{
        let go = GameObject::new_from_tex(tex
        );
        Player{
            go,
            pr: PhysRect::new(
                go.rect,
                max_v,
                weight
            ),
            acc,
            jump,
            frict,
            jumped: false,
        }
    }
    pub fn jumped(&self) -> bool {
        self.jumped
    }
    pub fn update(&mut self, time: &f64, input: &Input) {
        self.controls(input);
    }
    pub fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.go);
    }
}

impl Phys for Player {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
    fn post_physics(&mut self) {
        self.go.rect = self.pr.get_pixel_correct_rect();
    }
}

impl Player {
    fn controls(&mut self, input: &Input) {
        self.pr.a.x = 0.0;
        self.pr.a.y = self.acc.y;
        if input.left {
            self.pr.a.x -= self.acc.x;
            if self.go.tex_rect.w.signum() != -1.0 {
                self.pr.v.x = 0.0;
            }
            self.go.tex_rect.w = -(self.go.texture.width as f64);
        }
        if input.right {
            self.pr.a.x += self.acc.x;
            if self.go.tex_rect.w.signum() != 1.0 {
                self.pr.v.x = 0.0;
            }
            self.go.tex_rect.w = self.go.texture.height as f64;
        }
        if (!input.right && !input.left) ||
            (input.right && input.left) {
            self.pr.a.x = -self.pr.v.x*10.0;
        }
        if input.a && self.pr.y_collision {
            self.pr.a.y = 0.0;
            self.pr.v.y = self.jump;
            self.jumped = true;
        } else {
            self.jumped = false;
        }
        if !input.a && self.pr.v.y < 0.0 {
            self.pr.a.y *= self.frict;
        }
        /*println!("acc: {}", self.pr.a);
        println!("vel: {}", self.pr.v);
        println!("pos: {}", self.pr.s);
        println!("col: {} {}", self.pr.x_collision, self.pr.y_collision);*/
    }
}
