use crate::component::{
    Boss, CalculateOutOfBounds, ChangeSprite, Enemy, Position, Render, Shooter, Velocity,
};

use specs::{Builder, Entity, World};

use quicksilver::geom::{Rectangle, Shape, Vector};

const ENEMY_HEAD_HEIGHT: f32 = 10.;

pub fn get_enemy_head_body_area(self_area: Rectangle, position: Vector) -> (Rectangle, Rectangle) {
    let self_area = self_area.with_center(position);
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

pub fn create_walker(world: &mut World) -> Entity {
    if rand::random() {
        world
            .create_entity()
            .with(Position {
                position: Vector::new(-50.0, 432.0),
            })
            .with(Velocity {
                velocity: Vector::new(125.0, 0.0),
            })
            .with(Render {
                sprite: "andador_flipped".to_string(),
                bounding_box: None,
            })
    } else {
        world
            .create_entity()
            .with(Position {
                position: Vector::new(850.0, 432.0),
            })
            .with(Velocity {
                velocity: Vector::new(-125.0, 0.0),
            })
            .with(Render {
                sprite: "andador".to_string(),
                bounding_box: None,
            })
    }
    .with(CalculateOutOfBounds)
    .with(Enemy { score: 100 })
    .build()
}

pub fn create_shooter(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(CalculateOutOfBounds)
        .with(Position {
            position: Vector::new(850.0, 433.5),
        })
        .with(Velocity {
            velocity: Vector::new(-125.0, 0.0),
        })
        .with(Render {
            sprite: "atirador".to_string(),
            bounding_box: None,
        })
        .with(Enemy { score: 200 })
        .with(Shooter {
            maximum_fireballs: 2,
            fireball_amount: 0,
            coefficient_1: 0.175,
            coefficient_2: 0.0,
        })
        .build()
}

pub fn create_boss(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Boss {
            lives: 10,
            normal_lives: 5,
        })
        .with(Position {
            position: Vector::new(748.5, 428.0),
        })
        .with(Render {
            sprite: "chefe".to_string(),
            bounding_box: None,
        })
        .with(Enemy { score: 300 })
        .with(ChangeSprite {
            new_sprite: "chefeapelao".to_string(),
            do_change: false,
        })
        .with(Shooter {
            maximum_fireballs: 2,
            fireball_amount: 0,
            coefficient_1: 0.075,
            coefficient_2: -0.05,
        })
        .build()
}
