use std::time::Duration;

use crate::component::{CalculateOutOfBounds, Hero, Position, Render, Velocity};

use quicksilver::geom::{Rectangle, Shape, Vector};

use specs::{Builder, Entity, World};

const HERO_FEET_HEIGHT: f32 = 10.0;

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
