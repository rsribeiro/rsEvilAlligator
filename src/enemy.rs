use std::{cell::RefCell, rc::Rc};

use crate::{entity::Entity, hero::Hero};

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Atlas, Background::Img},
    lifecycle::{Asset, Window},
    Result,
};

const ENEMY_HEAD_HEIGHT: f32 = 10.;

fn get_enemy_head_body_area(self_area: Rectangle) -> (Rectangle, Rectangle) {
    (
        Rectangle::new(
            self_area.top_left(),
            Vector::new(self_area.width(), ENEMY_HEAD_HEIGHT),
        ),
        Rectangle::new(
            self_area.top_left() + Vector::new(0., 10),
            Vector::new(self_area.width(), self_area.height() - ENEMY_HEAD_HEIGHT),
        ),
    )
}

#[derive(PartialEq)]
enum EnemyDirection {
    Left,
    Right,
}

enum WalkerSprite {
    WalkingLeftSprite,  //andador
    WalkingRightSprite, //andador_flipped
}

pub struct Walker {
    atlas: Rc<RefCell<Asset<Atlas>>>,
    sprite: WalkerSprite,
    position: Vector,
    direction: EnemyDirection,
}

impl Walker {
    pub fn new(atlas: Rc<RefCell<Asset<Atlas>>>) -> Result<Walker> {
        if rand::random() {
            let position = Vector::new(-50., 414. + 18.);
            let direction = EnemyDirection::Right;
            Ok(Walker {
                atlas,
                sprite: WalkerSprite::WalkingRightSprite,
                position,
                direction,
            })
        } else {
            let position = Vector::new(850., 414. + 18.);
            let direction = EnemyDirection::Left;
            Ok(Walker {
                atlas,
                sprite: WalkerSprite::WalkingLeftSprite,
                position,
                direction,
            })
        }
    }
}

impl Entity for Walker {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let pos = self.position;
        let sprite = &self.sprite;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let image = loaded_atlas
                .get(match sprite {
                    WalkerSprite::WalkingRightSprite => "andador_flipped",
                    WalkerSprite::WalkingLeftSprite => "andador",
                })
                .unwrap()
                .unwrap_image();
            window.draw(&image.area().with_center(pos), Img(&image));
            Ok(())
        })
    }

    fn update(&mut self, window: &mut Window, _hero: &mut Hero) -> Result<()> {
        let interval: f32 = window.update_rate() as f32;
        self.position += match self.direction {
            EnemyDirection::Left => Vector::new(-0.125 * interval, 0.),
            EnemyDirection::Right => Vector::new(0.125 * interval, 0.),
        };
        Ok(())
    }

    fn collision(&mut self, hero: &mut Hero) -> Result<bool> {
        let self_pos = self.position;
        let hero_pos = hero.get_position();
        let mut stomp: bool = false;
        let mut kill_hero: bool = false;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let self_area = loaded_atlas
                .get("andador")
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(self_pos);
            let (enemy_head_area, enemy_body_area) = get_enemy_head_body_area(self_area);
            let hero_area = loaded_atlas
                .get("heroi")
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(*hero_pos);
            let (hero_body_area, hero_feet_area) = crate::hero::get_hero_body_feet_area(hero_area);

            stomp = enemy_head_area.overlaps(&hero_feet_area);
            kill_hero = enemy_body_area.overlaps(&hero_body_area);
            Ok(())
        })?;

        if stomp {
            hero.give_score(100);
            Ok(true)
        } else if kill_hero {
            hero.take_life();
            Ok(false)
        } else {
            Ok(false)
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.position.x < -100. || self.position.x > 900.
    }
}

const SHOOTER_MAX_FIREBALLS: i32 = 2;

pub struct Shooter {
    atlas: Rc<RefCell<Asset<Atlas>>>,
    position: Vector,
    fireballs: Vec<Box<Fireball>>,
    dead: bool,
}

impl Shooter {
    pub fn new(atlas: Rc<RefCell<Asset<Atlas>>>) -> Result<Shooter> {
        let position = Vector::new(850., 417. + 16.5);
        let fireballs = Vec::new();
        let dead = false;

        let mut shooter = Shooter {
            atlas,
            position,
            fireballs,
            dead,
        };

        for i in 0..SHOOTER_MAX_FIREBALLS {
            shooter.add_fireball(i)?;
        }

        Ok(shooter)
    }

    fn add_fireball(&mut self, i: i32) -> Result<()> {
        let randomness = rand::random::<f32>() / 12.;
        self.fireballs.push(Box::new(Fireball::new(
            Rc::clone(&self.atlas),
            self.position,
            (0.175 * (i + 1) as f32) + randomness,
        )?));
        Ok(())
    }
}

impl Entity for Shooter {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.dead {
            let pos = self.position;
            self.atlas.borrow_mut().execute(|loaded_atlas| {
                let image = loaded_atlas.get("atirador").unwrap().unwrap_image();
                window.draw(&image.area().with_center(pos), Img(&image));
                Ok(())
            })?;
        }

        for fireball in self.fireballs.iter_mut() {
            fireball.draw(window)?;
        }

        Ok(())
    }

    fn update(&mut self, window: &mut Window, hero: &mut Hero) -> Result<()> {
        if !self.dead {
            let interval: f32 = window.update_rate() as f32;
            self.position += Vector::new(-0.125 * interval, 0.);
        }

        for fireball in self.fireballs.iter_mut() {
            fireball.update(window, hero)?;
        }

        if !self.dead {
            let mut i = 0;
            while i != self.fireballs.len() {
                if self.fireballs[i].is_out_of_bounds() || self.fireballs[i].collision(hero)? {
                    self.fireballs.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        if self.fireballs.len() < SHOOTER_MAX_FIREBALLS as usize {
            self.add_fireball(SHOOTER_MAX_FIREBALLS - self.fireballs.len() as i32)?;
        }

        Ok(())
    }

    fn collision(&mut self, hero: &mut Hero) -> Result<bool> {
        if self.dead {
            Ok(false)
        } else {
            let self_pos = self.position;
            let hero_pos = hero.get_position();
            let mut stomp: bool = false;
            let mut kill_hero: bool = false;
            self.atlas.borrow_mut().execute(|loaded_atlas| {
                let self_area = loaded_atlas
                    .get("atirador")
                    .unwrap()
                    .unwrap_image()
                    .area()
                    .with_center(self_pos);
                let (enemy_head_area, enemy_body_area) = get_enemy_head_body_area(self_area);
                let hero_area = loaded_atlas
                    .get("heroi")
                    .unwrap()
                    .unwrap_image()
                    .area()
                    .with_center(*hero_pos);
                let (hero_body_area, hero_feet_area) =
                    crate::hero::get_hero_body_feet_area(hero_area);

                stomp = enemy_head_area.overlaps(&hero_feet_area);
                kill_hero = enemy_body_area.overlaps(&hero_body_area);
                Ok(())
            })?;

            if stomp {
                hero.give_score(200);
                self.dead = true;
                Ok(false)
            } else if kill_hero {
                hero.take_life();
                Ok(false)
            } else {
                Ok(false)
            }
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        let self_oob = self.position.x < -100. || self.position.x > 900.;
        let fireballs_oob = self.fireballs.iter().all(|f| f.is_out_of_bounds());
        (self.dead || self_oob) && fireballs_oob
    }
}

pub struct Fireball {
    atlas: Rc<RefCell<Asset<Atlas>>>,
    position: Vector,
    speed: f32,
}

impl Fireball {
    pub fn new(
        atlas: Rc<RefCell<Asset<Atlas>>>,
        initial_position: Vector,
        speed: f32,
    ) -> Result<Fireball> {
        Ok(Fireball {
            atlas,
            position: initial_position,
            speed,
        })
    }
}

impl Entity for Fireball {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let pos = self.position;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let image = loaded_atlas.get("tiro").unwrap().unwrap_image();
            window.draw(&image.area().with_center(pos), Img(&image));
            Ok(())
        })
    }

    fn update(&mut self, window: &mut Window, _hero: &mut Hero) -> Result<()> {
        let interval: f32 = window.update_rate() as f32;
        self.position += Vector::new(-self.speed * interval, 0.);
        Ok(())
    }

    fn collision(&mut self, hero: &mut Hero) -> Result<bool> {
        let self_pos = self.position;
        let hero_pos = hero.get_position();
        let mut overlaps: bool = false;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let self_area = loaded_atlas
                .get("tiro")
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(self_pos);
            let hero_area = loaded_atlas
                .get("heroi")
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(*hero_pos);
            overlaps = self_area.overlaps(&hero_area);
            Ok(())
        })?;
        if overlaps && hero.take_life() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.position.x < -100. || self.position.x > 900.
    }
}

enum BossSprite {
    Normal,
    Angry,
}

pub struct Boss {
    atlas: Rc<RefCell<Asset<Atlas>>>,
    position: Vector,
    fireballs: Vec<Box<Fireball>>,
    sprite: BossSprite,
    angry: bool,
    lives: i32,
}

const BOSS_MAX_FIREBALLS: i32 = 2;
const ANGRY_BOSS_MAX_FIREBALLS: i32 = 4;
const BOSS_NORMAL_SPRITE: &str = "chefe";
const BOSS_ANGRY_SPRITE: &str = "chefeapelao";
const BOSS_TOTAL_LIVES: i32 = 10;
const BOSS_NORMAL_LIVES: i32 = BOSS_TOTAL_LIVES / 2;

impl Boss {
    pub fn new(atlas: Rc<RefCell<Asset<Atlas>>>) -> Result<Boss> {
        let position = Vector::new(748.5, 428);
        let fireballs = Vec::new();
        let lives = BOSS_TOTAL_LIVES;
        let mut boss = Boss {
            atlas,
            position,
            fireballs,
            sprite: BossSprite::Normal,
            angry: false,
            lives,
        };
        for i in 0..BOSS_MAX_FIREBALLS {
            boss.add_fireball(i)?;
        }

        Ok(boss)
    }

    fn add_fireball(&mut self, i: i32) -> Result<()> {
        let randomness = rand::random::<f32>() / 12.;
        self.fireballs.push(Box::new(Fireball::new(
            Rc::clone(&self.atlas),
            self.position,
            (0.05 * (2 * (i + 1) - 1) as f32) + randomness,
        )?));
        Ok(())
    }

    fn make_angry(&mut self) {
        self.sprite = BossSprite::Angry;
        self.angry = true;
    }
}

impl Entity for Boss {
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let pos = self.position;
        let sprite = &self.sprite;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let image = loaded_atlas
                .get(match sprite {
                    BossSprite::Normal => BOSS_NORMAL_SPRITE,
                    BossSprite::Angry => BOSS_ANGRY_SPRITE,
                })
                .unwrap()
                .unwrap_image();
            window.draw(&image.area().with_center(pos), Img(&image));
            Ok(())
        })?;

        for fireball in self.fireballs.iter_mut() {
            fireball.draw(window)?;
        }

        Ok(())
    }

    fn update(&mut self, window: &mut Window, hero: &mut Hero) -> Result<()> {
        if !self.angry && self.lives == BOSS_NORMAL_LIVES {
            self.make_angry();
        }

        for fireball in self.fireballs.iter_mut() {
            fireball.update(window, hero)?;
        }

        let mut i = 0;
        while i != self.fireballs.len() {
            if self.fireballs[i].is_out_of_bounds() || self.fireballs[i].collision(hero)? {
                self.fireballs.remove(i);
            } else {
                i += 1;
            }
        }

        if self.angry {
            if self.fireballs.len() < ANGRY_BOSS_MAX_FIREBALLS as usize {
                self.add_fireball(ANGRY_BOSS_MAX_FIREBALLS - self.fireballs.len() as i32)?;
            }
        } else if self.fireballs.len() < BOSS_MAX_FIREBALLS as usize {
            self.add_fireball(BOSS_MAX_FIREBALLS - self.fireballs.len() as i32)?;
        }

        Ok(())
    }

    fn collision(&mut self, hero: &mut Hero) -> Result<bool> {
        let self_pos = self.position;
        let hero_pos = hero.get_position();
        let mut stomp: bool = false;
        let mut kill_hero: bool = false;
        let sprite = &self.sprite;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let self_area = loaded_atlas
                .get(match sprite {
                    BossSprite::Normal => BOSS_NORMAL_SPRITE,
                    BossSprite::Angry => BOSS_ANGRY_SPRITE,
                })
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(self_pos);
            let (enemy_head_area, enemy_body_area) = get_enemy_head_body_area(self_area);
            let hero_area = loaded_atlas
                .get("heroi")
                .unwrap()
                .unwrap_image()
                .area()
                .with_center(*hero_pos);
            let (hero_body_area, hero_feet_area) = crate::hero::get_hero_body_feet_area(hero_area);

            stomp = enemy_head_area.overlaps(&hero_feet_area);
            kill_hero = enemy_body_area.overlaps(&hero_body_area);

            Ok(())
        })?;

        if stomp {
            hero.give_score(300);
            hero.reset_position();
            self.lives -= 1;
            Ok(self.lives <= 0)
        } else if kill_hero {
            hero.take_life();
            Ok(false)
        } else {
            Ok(false)
        }
    }

    fn is_out_of_bounds(&self) -> bool {
        false
    }
}
