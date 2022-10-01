pub use crate::{
    camera::Camera,
    input::Input,
    GameObject,
    Colour,
    TextureManager,
    resource::Texture,
};

pub use geometry::*;

pub trait Draw {
    fn new<'sdl, TexType>(tm: &'sdl mut TextureManager<TexType>) -> Result<Self, String>
    where Self: Sized;
    fn update(&mut self, time: &f64, input: &Input);
    fn draw(&self, cam: &mut Camera);
}
