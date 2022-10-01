use crate::Colour;

///use super::helper::*;
use super::physics::*;

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
    pub fn new(phys: PhysRect) -> Self {
        let mut phys = phys;
        //phys.a.y = 100.0;
        phys.weight = 1.0;
        phys.friction = 0.95;
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
        if self.pr().last_update == LastUpdate::Y && self.pr().s.y > other.s.y {
            self.pr.a.y = 100.0;
        }
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
