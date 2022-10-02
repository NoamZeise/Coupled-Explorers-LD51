use std::ops;
use std::fmt;

///  A rectangle where x,y represents the coord of the upper left corner
#[derive(Clone, Copy)]
pub struct Rect {
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rect { x, y, w, h }
    }

    pub fn blank() -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn new_from_vec2s(v1 : &Vec2, v2 : &Vec2) -> Self {
        let mut smallest : Vec2 = *v1;
        let mut dim = Vec2::new(0.0, 0.0);

        if smallest.x > v2.x {
            smallest.x = v2.x;
            dim.x = v1.x - v2.x;
        } else {
            dim.x = v2.x - v1.x;
        }

        if smallest.y > v2.y {
            smallest.y = v2.y;
            dim.y = v1.y - v2.y;
        } else {
            dim.y = v2.y - v1.y;
        }
        
        Rect { x: smallest.x, y: smallest.y, w: dim.x, h: dim.y }
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }

    pub fn colliding(&self, rect : &Rect) -> bool {
        self.x < rect.x + rect.w &&
        self.x + self.w > rect.x &&
        self.y < rect.y + rect.h &&
        self.y + self.h > rect.y
    }

    pub fn contains(&self, vec : &Vec2) -> bool {
        self.x          < vec.x &&
        self.x + self.w > vec.x &&
        self.y          < vec.y &&
        self.y + self.h > vec.y
    }

    pub fn set_pos(&mut self, vec: &Vec2) {
        self.x = vec.x;
        self.y = vec.y;
    }

    pub fn floor(&self) -> Rect {
        Rect::new(
            self.x.floor(),
            self.y.floor(),
            self.w.floor(),
            self.h.floor(),
            )
    }
    pub fn round_pos(&self) -> Rect {
        Rect::new(
            self.x.round(),
            self.y.round(),
            self.w,
            self.h
        )
    }
}

impl ops::AddAssign<Vec2> for Rect {
    fn add_assign(&mut self, other : Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// A 2D Vector
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x : f64,
    pub y : f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, other : Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other : Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, other : Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other : f64) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}
impl ops::Mul<&f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other : &f64) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}
impl ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, other : f64) -> Vec2 {
        Vec2::new(self.x / other, self.y / other)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}  y: {}", self.x, self.y)
    }
}
