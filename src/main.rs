extern crate quicksilver;
extern crate rand;

mod enemy;
mod entity;
mod healing;
mod hero;
mod hud;
mod music;

use std::{cell::RefCell, rc::Rc};

use crate::{
    enemy::{Boss, Shooter, Walker},
    entity::Entity,
    healing::Healing,
    hero::Hero,
    hud::HUD,
    music::{Music, MusicPlayer},
};

use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Atlas, Background::Img, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};

const GAME_SCREEN_WIDTH: u32 = 800;
const GAME_SCREEN_HEIGHT: u32 = 600;
const BOSS_CYCLE: u32 = 11;
const NEW_BODY_CYCLE: u64 = 210;
const ICON: &str = "icone.png";

#[derive(PartialEq)]
enum GameState {
    Initialiazing,
    Running,
    GameOver,
}

enum Background {
    GameScenario,
    Victory,
    Defeat,
}

struct EvilAlligator {
    atlas: Rc<RefCell<Asset<Atlas>>>,
    background: Background,
    hero: Hero,
    state: GameState,
    cycle_timer: u64,
    cycle_counter: u32,
    entities: Vec<Box<Entity>>,
    music_player: MusicPlayer,
    hud: HUD,
}

impl State for EvilAlligator {
    fn new() -> Result<EvilAlligator> {
        let atlas = Rc::new(RefCell::new(Asset::new(Atlas::load(
            "evil_alligator.atlas",
        ))));
        let hero = Hero::new(Rc::clone(&atlas))?;
        let music_player = MusicPlayer::new()?;
        let hud = HUD::new()?;
        Ok(EvilAlligator {
            atlas,
            background: Background::GameScenario,
            hero,
            state: GameState::Initialiazing,
            cycle_timer: 0,
            cycle_counter: 0,
            entities: vec![],
            music_player,
            hud,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.state == GameState::Running {
            if self.cycle_counter < BOSS_CYCLE {
                if self.cycle_timer == 0 {
                    self.music_player.play_music(Music::NormalMusic)?;
                }
                self.cycle_timer += 1;
                if self.cycle_timer % NEW_BODY_CYCLE == 0 {
                    self.cycle_counter += 1;
                    if self.cycle_counter == BOSS_CYCLE {
                        self.music_player.play_music(Music::BossMusic)?;
                        let enemy = Boss::new(Rc::clone(&self.atlas))?;
                        self.entities.push(Box::new(enemy));
                    } else {
                        if self.cycle_counter % 2 == 1 {
                            let enemy = Walker::new(Rc::clone(&self.atlas))?;
                            self.entities.push(Box::new(enemy));
                        } else {
                            let enemy = Shooter::new(Rc::clone(&self.atlas))?;
                            self.entities.push(Box::new(enemy));
                        }
                        if self.cycle_counter % 3 == 0 {
                            let healing = Healing::new(Rc::clone(&self.atlas))?;
                            self.entities.push(Box::new(healing));
                        }
                    }
                }
            } else if self.entities.is_empty() {
                self.victory()?;
            }

            self.hero.update(window)?;

            for entity in self.entities.iter_mut() {
                entity.update(window, &mut self.hero)?;
            }

            let mut i = 0;
            while i != self.entities.len() {
                if self.entities[i].is_out_of_bounds()
                    || self.entities[i].collision(&mut self.hero)?
                {
                    self.entities.remove(i);
                } else {
                    i += 1;
                }
            }

            if self.hero.lives() <= 0 {
                self.defeat()?;
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let mut running = false;
        let background = &self.background;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let cenario = loaded_atlas
                .get(match background {
                    Background::GameScenario => "cenario",
                    Background::Defeat => "inferno",
                    Background::Victory => "ceu",
                })
                .unwrap()
                .unwrap_image();
            window.draw(&cenario.area().with_center((400, 300)), Img(&cenario));
            running = true;
            Ok(())
        })?;

        if self.state == GameState::Initialiazing && running {
            self.state = GameState::Running;
        }

        if self.state == GameState::Running {
            for entity in self.entities.iter_mut() {
                entity.draw(window)?;
            }
            self.hero.draw(window)?;
            self.hud
                .draw(window, self.hero.lives(), self.hero.score())?;
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match self.state {
            GameState::Running => {
                if let Event::Key(Key::Up, ButtonState::Pressed) = event {
                    self.hero.jump()?;
                }
                if let Event::Key(Key::Escape, ButtonState::Pressed) = event {
                    self.defeat()?;
                }
            }
            GameState::GameOver => {
                if let Event::Key(Key::Escape, ButtonState::Pressed) = event {
                    window.close();
                }
                if let Event::Key(Key::Return, ButtonState::Pressed) = event {
                    window.close();
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl EvilAlligator {
    fn defeat(&mut self) -> Result<()> {
        self.end_game()?;
        self.background = Background::Defeat;
        self.music_player.play_music(Music::GameOverMusic)?;
        Ok(())
    }

    fn victory(&mut self) -> Result<()> {
        self.end_game()?;
        self.background = Background::Victory;
        self.music_player.play_music(Music::VictoryMusic)?;
        Ok(())
    }

    fn end_game(&mut self) -> Result<()> {
        self.entities.clear();
        self.state = GameState::GameOver;
        Ok(())
    }
}

fn main() {
    run::<EvilAlligator>(
        "Evil Alligator",
        Vector::new(GAME_SCREEN_WIDTH, GAME_SCREEN_HEIGHT),
        Settings {
            icon_path: Some(ICON),
            ..Settings::default()
        },
    );
}
