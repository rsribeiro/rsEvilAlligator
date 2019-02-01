use std::time::Duration;

use crate::component::{CalculateOutOfBounds, Hero, Position, Render, Velocity};

use quicksilver::geom::{Rectangle, Shape, Vector};

use specs::{Builder, Entity, World};

// const HERO_HORIZONTAL_SPEED: f32 = 250.0;
// const HERO_VERTICAL_SPEED: f32 = 200.0;
// const HERO_INITIAL_LIVES: i32 = 5;
// const TOTAL_BLINK_TIME: f64 = 2000.0;
// const BLINK_INTERVAL: f64 = 200.0;
const HERO_FEET_HEIGHT: f32 = 10.0;

// pub struct Hero {
//     atlas: Rc<RefCell<Asset<Atlas>>>,
//     position: Vector,
//     initialized: bool,
//     jumping: bool,
//     lives: i32,
//     score: i32,
//     blinking: bool,
//     blinking_time: f64,
// }

pub fn get_hero_body_feet_area(self_area: Rectangle, position: Vector) -> (Rectangle, Rectangle) {
    let self_area = self_area.with_center(position);
    (
        Rectangle::new(
            self_area.top_left(),
            Vector::new(self_area.width(), self_area.height() - HERO_FEET_HEIGHT),
        ),
        Rectangle::new(
            self_area.top_left() + Vector::new(0., self_area.height() - HERO_FEET_HEIGHT),
            Vector::new(self_area.width(), HERO_FEET_HEIGHT),
        ),
    )
}

pub fn create_hero(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Hero {
            lives: 5,
            score: 0,
            blinking: false,
            render: true,
            reset_position: false,
            blink_timer: Duration::from_millis(0),
        })
        .with(CalculateOutOfBounds)
        .with(Position {
            position: Vector::new(425.0, 425.0),
        })
        .with(Velocity {
            velocity: Vector::new(0.0, 0.0),
        })
        .with(Render {
            sprite: "heroi".to_string(),
            bounding_box: None,
        })
        .build()
}

// impl Hero {
//     pub fn new(atlas: Rc<RefCell<Asset<Atlas>>>) -> Result<Hero> {
//         let position = Vector::new(425., 415.);
//         Ok(Hero {
//             atlas,
//             position,
//             jumping: false,
//             initialized: false,
//             lives: HERO_INITIAL_LIVES,
//             score: 0,
//             blinking: false,
//             blinking_time: 0.,
//         })
//     }

//     pub fn jump(&mut self) -> Result<()> {
//         if !self.jumping && (self.position.y - 425.).abs() < std::f32::EPSILON {
//             self.jumping = true;
//         }
//         Ok(())
//     }

//     pub fn lives(&self) -> i32 {
//         self.lives
//     }
//     pub fn give_life(&mut self) {
//         self.lives += 1;
//     }
//     pub fn score(&self) -> i32 {
//         self.score
//     }
//     pub fn give_score(&mut self, score: i32) {
//         self.score += score;
//     }
//     pub fn get_position(&self) -> &Vector {
//         &self.position
//     }

//     pub fn draw(&mut self, window: &mut Window) -> Result<()> {
//         if (self.blinking_time / BLINK_INTERVAL) as i32 % 2 == 0 || !self.blinking {
//             let pos = self.position;
//             self.atlas.borrow_mut().execute(|loaded_atlas| {
//                 let image = loaded_atlas.get("heroi").unwrap().unwrap_image();
//                 let area = image.area();
//                 window.draw(&area.with_center(pos), Img(&image));
//                 Ok(())
//             })?;
//             if !self.initialized {
//                 self.initialized = true;
//             }
//         }

//         Ok(())
//     }

//     pub fn update(&mut self, window: &mut Window) -> Result<()> {
//         let interval = window.update_rate();

//         if self.blinking {
//             self.blinking_time += interval;
//             if self.blinking_time >= TOTAL_BLINK_TIME {
//                 self.blinking_time = 0.;
//                 self.blinking = false;
//             }
//         }

//         let interval: f32 = interval as f32;
//         if window.keyboard()[Key::Left].is_down() && !window.keyboard()[Key::Right].is_down() {
//             self.position += Vector::new(-HERO_HORIZONTAL_SPEED * interval, 0.);
//         }
//         if window.keyboard()[Key::Right].is_down() && !window.keyboard()[Key::Left].is_down() {
//             self.position += Vector::new(HERO_HORIZONTAL_SPEED * interval, 0.);
//         }

//         if self.jumping {
//             self.position += Vector::new(0., -HERO_VERTICAL_SPEED * interval * 2.0);
//         } else {
//             self.position += Vector::new(0., HERO_VERTICAL_SPEED * interval);
//         }

//         if self.jumping && self.position.y < 300. {
//             self.jumping = false;
//         }

//         self.position = self
//             .position
//             .clamp(Vector::new(15., 0.), Vector::new(785., 425.));
//         Ok(())
//     }

//     pub fn take_life(&mut self) -> bool {
//         if !self.blinking {
//             self.lives -= 1;
//             self.blinking = true;
//             true
//         } else {
//             false
//         }
//     }

//     pub fn reset_position(&mut self) {
//         self.position = Vector::new(15., 300.);
//     }
// }
