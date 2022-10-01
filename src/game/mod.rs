mod map;
mod helper;
mod player;
mod world;
mod physics;

use helper::*;
use player::Player;
use physics::*;

enum Players {
    Quick,
    Heavy,
}

pub struct Game {
    quick: Player,
    heavy: Player,
    player: Players,
    objects: Vec<Box<dyn Phys>>,
    prev_input: Input,
}

impl Game {
    pub fn new<'sdl, TexType>(tm: &'sdl mut TextureManager<TexType>) -> Result<Game, String> {
        Ok(Game {
            quick: Player::new(tm.load("textures/quick.png")?, Vec2::new(1000.0, 700.0), -240.0, 3.0),
            heavy: Player::new(tm.load("textures/heavy.png")?, Vec2::new(800.0, 500.0), -160.0, 2.0),
            objects: vec![
                Box::new(world::StaticObs::new(
                    PhysRect::new_from_rect(Rect::new(0.0, 10.0, 10.0, 200.0))
                )),
                Box::new(world::StaticObs::new(
                    PhysRect::new_from_rect(Rect::new(10.0, 100.0, 130.0, 10.0))
                )),
                Box::new(world::GravObs::new(
                    PhysRect::new_from_rect(
                        Rect::new(100.0, 10.0, 20.0, 20.0)
                    )
                )),
                Box::new(world::BrittleObs::new(
                    PhysRect::new_from_rect(
                        Rect::new(200.0, 100.0, 20.0, 20.0)
                    )
                )),
                Box::new(world::DownObs::new(
                    PhysRect::new_from_rect(
                        Rect::new(160.0, 100.0, 20.0, 20.0)

                    )
                ))],
            player: Players::Heavy,
            prev_input: Input::new(),
        })
    }
    
    pub fn update(&mut self, time: &f64, input: &Input) {
        if input.debug_1 && !self.prev_input.debug_1{
            match self.player {
                Players::Heavy => self.player = Players::Quick,
                Players::Quick => self.player = Players::Heavy,
            }
        }
        match self.player {
            Players::Quick => {
                self.quick.update(time, input);
                phys_update(&mut self.objects, time, &mut self.quick);
            },
            Players::Heavy => {
                self.heavy.update(time, input);
                phys_update(&mut self.objects, time, &mut self.heavy);
            },
        }
        self.prev_input = *input;
    }

    pub fn draw(&self, cam: &mut Camera) {
        self.quick.draw(cam);
        self.heavy.draw(cam);
        for o in self.objects.iter() {
            cam.draw_rect(o.pr_im().get_pixel_correct_rect(), o.pr_im().colour);
        }
    }
}

fn phys_update(objs: &mut Vec<Box<dyn Phys>>, time: &f64, p: &mut Player) {
    for o in objs.iter_mut() {
        o.pre_physics();
    }
    p.pre_physics();
    for o in objs.iter_mut() {
        o.phys_x(time);
    }
    p.phys_x(time);
    collision_checks(objs);
    for o in objs.iter_mut() {
        collision_update(o.as_mut(), p);
    }
    for o in objs.iter_mut() {
        o.phys_y(time);
    }
    p.phys_y(time);
    collision_checks(objs);
    for o in objs.iter_mut() {
        collision_update(o.as_mut(), p);
    }
    for o in objs.iter_mut() {
        o.post_physics();
    }
    p.post_physics();
}

fn collision_checks(objs: &mut Vec<Box<dyn Phys>>) {
    for i in 0..objs.len() - 1 {
        for j in i + 1..objs.len() {
            let a : &mut Box<dyn Phys>;
            let b : &mut Box<dyn Phys>;
            // i and j are guarenteed to be different elements of objs
            // and within bounds of objs
            unsafe {
                a = &mut *(objs.as_mut_slice().get_unchecked_mut(i) as *mut _);
                b = &mut *(objs.as_mut_slice().get_unchecked_mut(j) as *mut _);
            }
            collision_update(a.as_mut(), b.as_mut());
        }
    }
}
