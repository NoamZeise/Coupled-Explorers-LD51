use crate::Colour;

use super::physics::*;
use geometry::*;

const WIDTH: f64 = 3.0;

pub fn get_brittle(rect: Rect) -> Vec<BrittleObs> {
    let mut obs = Vec::new();
    let width = WIDTH;
    let mut x = rect.x;
    while x < rect.x + rect.w - WIDTH {
        obs.push(
            BrittleObs::new(
                Rect::new(
                    x,
                    rect.y,
                    width,
                    rect.h,
                )
            )
        );
        x += width;
    }
    obs.push(
            BrittleObs::new(
                Rect::new(
                    x,
                    rect.y,
                    (rect.x + rect.w) - x,
                    rect.h,
                )
            )
        );
    obs
}

pub struct Nested {
    pr: PhysRect,
    col: Option<Rect>,
}

impl Nested {
    pub fn new(rect: Rect) ->  Self {
        Nested {
            pr: PhysRect::new_from_rect(rect),
            col: None,
        }
    }
    pub fn had_col(&self) -> bool {
        self.col.is_some()
    }
    pub fn add(old: Nested, objs: &mut Vec<Box<dyn Phys>>, nested: &mut Vec<Nested>) {
        let col = old.col.unwrap();
        let rect = old.pr.rect;
        let x = (if col.x > rect.x { col.x } else { rect.x }).floor();
        let w = (if rect.x + rect.w > col.x + col.w {
            (col.x + col.w) - x
        } else {
            (rect.x + rect.w) - x
        }).ceil();
        let brittle = Rect::new(x, rect.y, w, rect.h);
        for o in get_brittle(brittle) {
            objs.push(Box::new(o));
        }
        if x > rect.x {
            nested.push(
                Nested::new(
                    Rect::new(
                        rect.x,
                        rect.y,
                        x - rect.x,
                        rect.h
                    )
                )
            );
        }
        if x+w < rect.x + rect.w {
            nested.push(
                Nested::new(
                    Rect::new(
                        x+w,
                        rect.y,
                        (rect.x + rect.w) - (x + w),
                        rect.h
                    )
                )
            );
        }
    }
}

impl Phys for Nested {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
    fn collision(&mut self, other: &PhysRect) {
        if self.pr().last_update == LastUpdate::Y && self.pr().s.y > other.s.y {
            self.col = Some(other.rect);
        }
    }  
}


pub struct StaticObs {
    pub pr: PhysRect,
}

impl StaticObs {
    pub fn new(pr: PhysRect) -> Self {
        let mut pr = pr;
        pr.colour = Colour::new(30, 50, 30, 255);
        Self {
            pr
        }
    }
}

impl Phys for StaticObs {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
    fn collision(&mut self, _: &PhysRect) {}
}

pub struct GravObs {
    pub pr: PhysRect,
}

impl GravObs {
    pub fn new(phys: PhysRect) -> Self {
        let mut phys = phys;
        phys.a.y = 200.0;
        phys.weight = 10.0;
        phys.friction = 0.95;
        phys.colour = Colour::new(50, 30, 30, 255);
        GravObs { pr: phys }
    }
}

impl Phys for GravObs {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
}

pub struct BrittleObs {
    pub pr: PhysRect,
}

impl BrittleObs {
    pub fn new(rect: Rect) -> Self {
        let mut phys = PhysRect::new_from_rect(rect);
        //phys.a.y = 100.0;
        phys.weight = 1.0;
        phys.friction = 0.95;
        phys.a.y = 10.0;
        Self { pr: phys }
    }
}

impl Phys for BrittleObs {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
    fn collision(&mut self, other: &PhysRect) {
      //  if self.pr().last_update == LastUpdate::Y && self.pr().s.y > other.s.y {
      //      self.pr.a.y = 50.0;
      //  }
    }
}

pub struct DownObs {
    pub pr: PhysRect,
}

impl DownObs {
    pub fn new(phys: PhysRect) -> Self {
        let mut phys = phys;
        //phys.a.y = 100.0;
        phys.weight = 0.1;
        phys.friction = 0.95;
        Self { pr: phys }
    }
}

impl Phys for DownObs {
    fn pr(&mut self) -> &mut PhysRect {
        &mut self.pr
    }
    fn pr_im(&self) -> &PhysRect {
        &self.pr
    }
}
