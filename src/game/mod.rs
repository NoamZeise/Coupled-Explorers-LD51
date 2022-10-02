mod map;
mod helper;
mod player;
mod world;
mod physics;

use helper::*;
use player::Player;
use physics::*;
use world::*;
use map::*;

use std::collections::HashMap;

const SWITCH_TIME: f64 = 10.0;

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum Players {
    Quick,
    Heavy,
}

pub struct Game {
    player: HashMap<Players, Player>,
    maps: Vec<Map>,
    m: usize,
    p : Players,
    finished_count: u8,
    player_spawn: Vec2,
    objects: Vec<Box<dyn Phys>>,
    nested: Vec<world::Nested>,
    prev_input: Input,
    map_loaded: bool,
    cam_returned: bool,
    switch_time: f64,
    level_complete: bool,
}

impl Game {
    pub fn new<'sdl, TexType>(tm: &'sdl mut TextureManager<TexType>) -> Result<Game, String> {
        let mut player = HashMap::new();
        player.insert(
            Players::Quick,
            Player::new(
                tm.load("textures/quick.png")?,
                Vec2::new(500.0, 550.0),
                -240.0, 3.0,
                Vec2::new(150.0, 800.0),
                2.0
            )
        );
        player.insert(
            Players::Heavy,
            Player::new(tm.load("textures/heavy.png")?,
                        Vec2::new(400.0, 500.0),
                        -160.0, 2.0,
                        Vec2::new(100.0, 800.0),
                        2.0
            )
        );

        let mut maps = Vec::new();
        for i in 0..6 {
            maps.push(Map::new(&("maps/".to_owned() + &i.to_string() + ".tmx"), tm)?);
        }
        
        let mut g = Game {
            player,
            maps,
            objects : Vec::new(),
            p: Players::Heavy,
            m: 0,
            prev_input: Input::new(),
            nested : Vec::new(),
            player_spawn: Vec2::new(0.0, 0.0),
            finished_count: 0,
            map_loaded: false,
            cam_returned: true,
            switch_time: 0.0,
            level_complete: false,
        };
        g.load_map();
        g.cam_returned = true;
        Ok(g)
    }

    pub fn jumped(&self) -> Option<Players> {
        if self.player[&self.p].jumped() {
            Some(self.p)
        } else {
            None
        }
    }

    pub fn game_complete(&self) -> bool {
        self.level_complete
    }
    
    pub fn update(&mut self, time: &f64, input: &Input) {
        if input.debug_1 && !self.prev_input.debug_1{
            self.swap_player();
        }
        if input.debug_2 && !self.prev_input.debug_2{
            self.next_lvl();
        }
        if input.debug_3 && !self.prev_input.debug_3{
            self.m = if self.m == 0 { 0 } else { self.m - 1}; 
            self.load_map();
        }

        if !self.cam_returned { return; }

        if input.restart && !self.prev_input.restart {
            self.load_map();
        }

        self.switch_time += time;
        if self.switch_time > SWITCH_TIME {
            self.switch_time = 0.0;
            self.swap_player();
        }
        
        self.player.get_mut(&self.p).unwrap().update(time, input);
        phys_update(
            &mut self.objects, time,
            &mut self.player.get_mut(&self.p).unwrap(),
            &mut self.nested
        );
        self.check_destroyed();
        self.prev_input = *input;
        //println!("phys: {}     nest: {}", self.objects.len(), self.nested.len()); 
    }

    pub fn draw(&mut self, cam: &mut Camera) {
        let p = self.player[&self.p].pr_im().get_pixel_correct_rect().centre();
        if self.map_loaded {
            cam.target_centre_pos(p, self.maps[self.m].tiled_map.rect);
            self.map_loaded = false;
            self.cam_returned = false;
        } else {
            if cam.done() || self.cam_returned {
                cam.centre_on_pos(p, self.maps[self.m].tiled_map.rect);
                self.cam_returned = true;
            }
        }
        
        self.maps[self.m].draw(cam);
        if self.finished_count == 0 {
            for (a, p) in self.player.iter_mut() {
                if *a == self.p {
                    p.go.colour.r = 255;
                    p.go.colour.a = 255;
                } else {
                    p.go.colour.r = 0;
                    p.go.colour.a = 170;
                }
                p.draw(cam);
            }
        } else {
            if self.cam_returned {
                self.player.get_mut(&self.p).unwrap().go.colour.a = 255;
                self.player.get_mut(&self.p).unwrap().go.colour.r = 255;
            }
            self.player[&self.p].draw(cam);
        }
        for o in self.objects.iter() {
            cam.draw_rect(o.pr_im().get_pixel_correct_rect(), o.pr_im().colour);
        }
        for n in self.nested.iter() {
            cam.draw_rect(n.pr_im().rect, n.pr_im().colour);
        }
        if self.finished_count == 0 {
            cam.draw_rect_static(
                Rect::new(
                    0.0, 154.0,
                    240.0,
                    10.0),
                Colour::new(100, 100, 100, 170));
            cam.draw_rect_static(
                Rect::new(
                    0.0, 154.0,
                    240.0 * (1.0 -(self.switch_time / SWITCH_TIME)),
                    10.0),
                Colour::new(0, 0, 0, 190));
        }
    }

    fn next_lvl(&mut self) {
        if self.m < self.maps.len() - 1 {
            self.m += 1;
            self.load_map();
        } else { self.level_complete = true; }
        self.map_loaded = false;
        self.cam_returned = true;
    }

    fn load_map(&mut self) {
        self.map_loaded = true;
        self.cam_returned = false;
        self.p = Players::Heavy;
        self.switch_time = 0.0;
        self.finished_count = 0;
        self.nested.clear();
        self.objects.clear();
        for ob_g in self.maps[self.m].tiled_map.obj_groups.iter() {
            for o in ob_g.objs.iter() {
                if check_bool("nested", &o.props) || check_bool("nested", &ob_g.props) {
                    self.nested.push(
                        Nested::new(o.rect.floor())
                    );
                }
                if check_bool("static", &o.props) || check_bool("static", &ob_g.props) {
                    self.objects.push(
                        Box::new(StaticObs::new(o.rect.floor()))
                    );
                }
                if check_bool("fall", &o.props) || check_bool("fall", &ob_g.props) {
                    self.objects.push(
                        Box::new(DownObs::new(o.rect.floor()))
                    );
                }
                if check_bool("push", &o.props) || check_bool("push", &ob_g.props) {
                    self.objects.push(
                        Box::new(GravObs::new(
                            o.rect.floor(),
                            get_int("weight", &o.props, 10) as f64 / 10.0
                        ))
                    );
                }
            }
            for p in ob_g.points.iter() {
                if check_bool("spawn", &p.props) {
                    self.player_spawn = Vec2::new(p.rect.x, p.rect.y);
                }
            }
        }
        for (_, p) in self.player.iter_mut() {
            p.pr().set_pos(self.player_spawn);
            p.pr().v = Vec2::new(0.0, 0.0);
            p.post_physics();
        }
    }

    fn swap_player(&mut self) {
        if self.finished_count > 0 { return; }
        self.cam_returned = false;
        self.map_loaded = true;
        self.p = match self.p {
            Players::Heavy => Players::Quick,
            Players::Quick => Players::Heavy,
        }
    }

    fn check_destroyed(&mut self) {
        let mut i: i32 = 0;
        while (i as usize) < self.objects.len() {
            if self.objects[i as usize].pr().rect.y >
                self.maps[self.m].tiled_map.rect.h + self.objects[i as usize].pr().rect.h {
                self.objects.remove(i as usize);
                i-=1;
            }
            i+=1;
        }
    
        if self.player[&self.p].pr_im().rect.y >
            self.maps[self.m].tiled_map.rect.h + self.player[&self.p].pr_im().rect.h {
            self.load_map();
        }
        if self.player[&self.p].pr_im().rect.x >
            self.maps[self.m].tiled_map.rect.w + self.player[&self.p].pr_im().rect.w {
                self.swap_player();
                self.finished_count += 1;
                if self.finished_count == 2 {
                    self.next_lvl();
                } 
        }
    }
}

fn phys_update(
    objs: &mut Vec<Box<dyn Phys>>, time: &f64,
    p: &mut Player,
    nested: &mut Vec<Nested>,
) {
    for o in objs.iter_mut() {
        o.pre_physics();
    }
    for n in nested.iter_mut() {
        n.pre_physics()
    }
    p.pre_physics();
    
    for o in objs.iter_mut() {
        o.phys_x(time);
    }
    for n in nested.iter_mut() {
        n.phys_x(time);
    }
    p.phys_x(time);
    
    coll(objs, p, nested);
    
    for o in objs.iter_mut() {
        o.phys_y(time);
    }
    for n in nested.iter_mut() {
        n.phys_y(time);
    }
    p.phys_y(time);
    
    coll(objs, p, nested);
    
    for o in objs.iter_mut() {
        o.post_physics();
    }
    for n in nested.iter_mut() {
        n.post_physics();
    }
    p.post_physics();

    let mut i: i32 = 0;
    while (i as usize) < nested.len() {
        if nested[i as usize].had_col() {
            let collided = nested.remove(i as usize);
            Nested::add(collided, objs, nested);
            i-=1;
        }
        i+=1;
    }
}

pub fn coll(
    objs: &mut Vec<Box<dyn Phys>>,
    p: &mut Player,
    nested: &mut Vec<Nested>,
)  {
    collision_checks(objs);
    for o in objs.iter_mut() {
        collision_update(o.as_mut(), p);
    }
    for n in nested.iter_mut() {
        collision_update(n, p);
    }
}

fn collision_checks(objs: &mut Vec<Box<dyn Phys>>) {
    if objs.len() == 0 { return; }
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

