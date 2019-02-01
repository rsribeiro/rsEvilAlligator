extern crate quicksilver;
extern crate rand;
extern crate specs;
#[macro_use]
extern crate specs_derive;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;

mod component;
mod enemy;
mod healing;
mod hero;
mod instant;
mod music;
mod resources;
mod system;

use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    component::{
        Background, Boss, CalculateOutOfBounds, ChangeSprite, Enemy, Fireball, Healing, Hero,
        Label, Position, Render, Shooter, Velocity,
    },
    instant::Instant,
    music::{Music, MusicPlayer},
    resources::{DeltaTime, KeyboardKeys, LabelVariable, PressedKeys, VariableDictionary},
    system::{
        CollisionSystem, FireballSystem, HeroBlinkingSystem, HeroControlSystem, LabelRenderSystem,
        OutOfBoundsSystem, RenderSystem, WalkSystem,
    },
};

use quicksilver::{
    geom::Vector,
    graphics::{Atlas, Color, Font, FontStyle},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Result,
};

use specs::{
    world::{EntitiesRes, Index},
    BitSet, Builder, Entity, RunNow, World,
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

struct EvilAlligator {
    world: World,
    atlas: Rc<RefCell<Asset<Atlas>>>,
    font: Rc<RefCell<Asset<Font>>>,
    hero: Entity,
    pressed_keys: BitSet,
    state: GameState,
    cycle_timer: u64,
    cycle_counter: u32,
    music_player: MusicPlayer,
    last_instant: Instant,
}

impl State for EvilAlligator {
    fn new() -> Result<EvilAlligator> {
        let atlas = Rc::new(RefCell::new(Asset::new(Atlas::load(
            "evil_alligator.atlas",
        ))));
        let font = Rc::new(RefCell::new(Asset::new(Font::load("cmunrm.ttf"))));
        let music_player = MusicPlayer::new()?;

        let mut world = World::new();
        register_components(&mut world);
        add_resorces(&mut world);

        create_background(&mut world, "cenario".to_string());
        create_label(
            &mut world,
            LabelVariable::FramesPerSecond,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(20, 587),
        );
        create_label(
            &mut world,
            LabelVariable::HeroLives,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(10, 20),
        );
        create_label(
            &mut world,
            LabelVariable::Score,
            FontStyle::new(48.0, Color::BLACK),
            Vector::new(730, 20),
        );
        let hero = hero::create_hero(&mut world);

        Ok(EvilAlligator {
            world,
            atlas,
            font,
            hero,
            pressed_keys: BitSet::new(),
            state: GameState::Initialiazing,
            cycle_timer: 0,
            cycle_counter: 0,
            music_player,
            last_instant: Instant::now(),
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.state == GameState::Running {
            let now = Instant::now();
            let time_step = now.duration_since(self.last_instant.clone());
            self.last_instant = now;
            {
                let mut delta = self.world.write_resource::<DeltaTime>();
                *delta = DeltaTime {
                    duration: time_step,
                };
            }

            {
                let self_pressed_keys = self.pressed_keys.clone();
                let mut pressed_keys = self.world.write_resource::<PressedKeys>();
                *pressed_keys = PressedKeys {
                    pressed_keys: self_pressed_keys,
                };
            }

            HeroControlSystem.run_now(&self.world.res);
            WalkSystem.run_now(&self.world.res);
            FireballSystem.run_now(&self.world.res);
            CollisionSystem.run_now(&self.world.res);
            OutOfBoundsSystem.run_now(&self.world.res);
            HeroBlinkingSystem.run_now(&self.world.res);

            if self.cycle_counter < BOSS_CYCLE {
                if self.cycle_timer == 0 {
                    self.music_player.play_music(Music::NormalMusic)?;
                }
                self.cycle_timer += 1;
                if self.cycle_timer % NEW_BODY_CYCLE == 0 {
                    self.cycle_counter += 1;
                    if self.cycle_counter == BOSS_CYCLE {
                        self.music_player.play_music(Music::BossMusic)?;
                        enemy::create_boss(&mut self.world);
                    } else {
                        if self.cycle_counter % 2 == 1 {
                            enemy::create_walker(&mut self.world);
                        } else {
                            enemy::create_shooter(&mut self.world);
                        }
                        if self.cycle_counter % 3 == 0 {
                            healing::create_healing_potion(&mut self.world);
                        }
                    }
                }
            } /* else if self.cycle_counter > BOSS_CYCLE {
                  if self.world.read_storage::<Boss>().count() == 0 {
                      self.victory()?;
                  }
              }*/
        }
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let mut running = self.state == GameState::Running;
        if !running {
            self.atlas.borrow_mut().execute(|_| {
                running = true;
                Ok(())
            })?;

            if self.state == GameState::Initialiazing && running {
                self.state = GameState::Running;
            } else if self.state == GameState::Initialiazing && !running {
                return Ok(());
            }
        }

        RenderSystem::new(window, Rc::clone(&self.atlas))?.run_now(&self.world.res);
        if self.state == GameState::Running {
            self.update_labels(window)?;
            LabelRenderSystem::new(window, Rc::clone(&self.font))?.run_now(&self.world.res);
        }
        self.world.maintain();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match self.state {
            GameState::Running => {
                match event {
                    Event::Key(Key::Up, ButtonState::Pressed) => {
                        self.pressed_keys.add(KeyboardKeys::KeyUp as u32);
                    }
                    Event::Key(Key::Up, ButtonState::Released) => {
                        self.pressed_keys.remove(KeyboardKeys::KeyUp as u32);
                    }
                    Event::Key(Key::Left, ButtonState::Pressed) => {
                        self.pressed_keys.add(KeyboardKeys::KeyLeft as u32);
                    }
                    Event::Key(Key::Left, ButtonState::Released) => {
                        self.pressed_keys.remove(KeyboardKeys::KeyLeft as u32);
                    }
                    Event::Key(Key::Right, ButtonState::Pressed) => {
                        self.pressed_keys.add(KeyboardKeys::KeyRight as u32);
                    }
                    Event::Key(Key::Right, ButtonState::Released) => {
                        self.pressed_keys.remove(KeyboardKeys::KeyRight as u32);
                    }
                    _ => {}
                };

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
        create_background(&mut self.world, "inferno".to_string());
        self.music_player.play_music(Music::GameOverMusic)?;
        Ok(())
    }

    fn victory(&mut self) -> Result<()> {
        self.end_game()?;
        create_background(&mut self.world, "ceu".to_string());
        self.music_player.play_music(Music::VictoryMusic)?;
        Ok(())
    }

    fn end_game(&mut self) -> Result<()> {
        self.world.delete_all();
        self.state = GameState::GameOver;
        Ok(())
    }

    fn update_labels(&mut self, window: &Window) -> Result<()> {
        let hero_storage = self.world.read_storage::<Hero>();
        if let Some(hero) = hero_storage.get(self.hero) {
            let mut dict = self.world.write_resource::<VariableDictionary>();
            *dict = VariableDictionary {
                dictionary: [
                    (
                        LabelVariable::FramesPerSecond,
                        format!("{:.0}", window.average_fps()),
                    ),
                    (LabelVariable::HeroLives, format!("{}", hero.lives)),
                    (LabelVariable::Score, format!("{}", hero.score)),
                ]
                .iter()
                .cloned()
                .collect(),
            }
        }
        Ok(())
    }
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Render>();
    world.register::<Shooter>();
    world.register::<Label>();
    world.register::<Hero>();
    world.register::<Boss>();
    world.register::<ChangeSprite>();
    world.register::<Enemy>();
    world.register::<Healing>();
    world.register::<Background>();
    world.register::<CalculateOutOfBounds>();
    world.register::<Fireball>();
}

fn add_resorces(world: &mut World) {
    world.add_resource(DeltaTime {
        duration: Duration::new(0, 0),
    });
    world.add_resource(VariableDictionary {
        dictionary: [
            (LabelVariable::FramesPerSecond, "60".to_string()),
            (LabelVariable::HeroLives, "5".to_string()),
            (LabelVariable::Score, "0".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    });
    world.add_resource(PressedKeys {
        pressed_keys: BitSet::new(),
    });
}

fn create_background(world: &mut World, sprite: String) -> Entity {
    world
        .create_entity()
        .with(Background)
        .with(Position {
            position: Vector::new(400, 300),
        })
        .with(Render {
            sprite: sprite,
            bounding_box: None,
        })
        .build()
}

fn create_label(
    world: &mut World,
    variable: LabelVariable,
    font_style: FontStyle,
    position: Vector,
) -> Entity {
    world
        .create_entity()
        .with(Label {
            bind_variable: variable,
            font_style: font_style,
        })
        .with(Position { position: position })
        .build()
}

fn main() {
    run::<EvilAlligator>(
        "Evil Alligator",
        Vector::new(GAME_SCREEN_WIDTH, GAME_SCREEN_HEIGHT),
        Settings {
            icon_path: Some(ICON),
            show_cursor: false,
            ..Settings::default()
        },
    );
}
