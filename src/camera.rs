use geometry::*;
use crate::{TextureDraw, GameObject, Colour};
use std::vec::Drain;

pub struct RectDraw {
    pub rect: Rect,
    pub colour: Colour,
}

pub enum CamDraw {
    Rect(RectDraw),
    Tex(TextureDraw),
}

pub struct Camera {
    target: Vec2,
    rect: Rect,
    true_rect: Rect,
    window_size: Vec2,
    size_ratio: Vec2,
    draws : Vec<CamDraw>,
    speed : f64,
    done: bool,
}

impl Camera {
    pub fn new(rect: Rect, window_size: Vec2) -> Camera {
        let mut cam = Camera {
            rect,
            target: Vec2::new(rect.x, rect.y),
            window_size,
            draws: Vec::new(),
            size_ratio: Vec2::new(0.0, 0.0),
            speed: 6.0,
            true_rect: rect,
            done: true,
        };
        cam.update_size_ratio();
        cam
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn update(&mut self, time: &f64) {
        let vec_to_target = self.target - self.true_rect.top_left();
        let change = vec_to_target * self.speed * time;
        if vec_to_target.x.abs() < 0.5 && vec_to_target.y.abs() < 0.5 {
            self.true_rect.x = self.target.x;
            self.true_rect.y = self.target.y;
            self.done = true;
        } else {
            self.done = false;
            self.true_rect += change;
        }
        self.rect = self.true_rect.round_pos();
    }

    pub fn drain_draws(&mut self) -> Drain<CamDraw> { 
        self.draws.drain(..)
    }
    
    pub fn draw(&mut self, game_obj: &GameObject) {
        self.draws.push(
            CamDraw::Tex(TextureDraw::new(
                game_obj.texture,
                Rect::new(
                    (game_obj.rect.x - (self.rect.x * game_obj.parallax.x)) / self.size_ratio.x,
                    (game_obj.rect.y - (self.rect.y * game_obj.parallax.y)) / self.size_ratio.y,
                    game_obj.rect.w / self.size_ratio.x,
                    game_obj.rect.h / self.size_ratio.y,
                ),
                game_obj.tex_rect,
                game_obj.colour,
            )
        ));
    }

    pub fn draw_rect(&mut self, rect: Rect, colour: Colour) {
        self.draws.push(
            CamDraw::Rect(RectDraw {
                rect: Rect::new(
                    (rect.x - self.rect.x) / self.size_ratio.x,
                    (rect.y - self.rect.y) / self.size_ratio.y,
                    rect.w / self.size_ratio.x,
                    rect.h / self.size_ratio.y,
                ),
                colour,
            }
        ));
    }

    pub fn draw_rect_static(&mut self, rect: Rect, colour: Colour) {
        self.draws.push(
            CamDraw::Rect(RectDraw {
                rect: Rect::new(
                    rect.x / self.size_ratio.x,
                    rect.y / self.size_ratio.y,
                    rect.w / self.size_ratio.x,
                    rect.h / self.size_ratio.y,
                ),
                colour,
            }
        ));
    }

    pub fn get_offset(&self) -> Vec2 {
        return Vec2::new(self.rect.x, self.rect.y);
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.rect.x = offset.x;
        self.rect.y = offset.y;
    }

    fn calc_offset(cam: f64, current: f64, min: f64, max: f64) -> f64 {
        if cam > max {
            return (max/2.0) - min;
        }
        let min = min + cam/2.0;
        let max = min + max - cam/1.0;
        if current > min && current < max {
            current
        } else if current < min {
            min 
        } else if current > max {
            max
        } else {
            current
        }
    }

    fn calc_vec2_off(&self, p: Vec2, lim: Rect) -> Vec2 {
        Vec2::new(
            Self::calc_offset(
                self.rect.w,
                p.x - (self.rect.w/2.0),
                lim.x - (self.rect.w/2.0),
                lim.w,
            ),
            Self::calc_offset(
                self.rect.h,
                p.y - (self.rect.h/2.0),
                lim.y - (self.rect.h/2.0),
                lim.h
            )
        )
    }

    pub fn centre_on_pos(&mut self, p: Vec2, lim: Rect) {
        let v = self.calc_vec2_off(p, lim);
        self.rect.x = v.x;
        self.rect.y = v.y;
        self.target.x = v.x;
        self.target.y = v.y;
    }
    pub fn target_centre_pos(&mut self, p: Vec2, lim: Rect) {
        let v = self.calc_vec2_off(p, lim);
        self.target.x = v.x;
        self.target.y = v.y;
    }

    pub fn get_window_size(&self) -> Vec2 {
        self.window_size
    }

    pub fn set_window_size(&mut self, size: Vec2) {
        self.window_size = size;
        self.update_size_ratio();
    }

    pub fn get_view_size(&self) -> Vec2 {
        Vec2::new(self.rect.w, self.rect.h)
    }
    pub fn set_view_size(&mut self, view: Vec2) {
        self.rect.w = view.x;
        self.rect.h = view.y;
        self.update_size_ratio();
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.rect.w / self.rect.h
    }

    fn update_size_ratio(&mut self) {
        self.size_ratio = Vec2::new(
                self.rect.w / self.window_size.x,
                self.rect.h / self.window_size.y
        );
    }
}
