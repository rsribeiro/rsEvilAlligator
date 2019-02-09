use crate::component::{
    Boss, CalculateOutOfBounds, ChangeSprite, Enemy, Fireball, Position, Render, Shooter, Velocity,
};

use specs::{Builder, World};

use quicksilver::geom::{Rectangle, Shape, Vector};

use rand::{thread_rng, Rng};

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

pub fn create_walker(world: &mut World) {
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
    .build();
}

pub fn create_shooter(world: &mut World) {
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
            projectile_sprite: "tiro".to_string(),
            maximum_fireballs: 2,
            fireball_amount: 0,
            coefficient_1: 0.175,
            coefficient_2: 0.0,
        })
        .build();
}

pub fn create_flyer(world: &mut World) {
    world
        .create_entity()
        .with(CalculateOutOfBounds)
        .with(Position {
            position: Vector::new(850.0, 400.0),
        })
        .with(Velocity {
            velocity: Vector::new(-150.0, 0.0),
        })
        .with(Render {
            sprite: "alma".to_string(),
            bounding_box: None,
        })
        .with(Enemy { score: 200 })
        .with(Shooter {
            projectile_sprite: "tiro".to_string(),
            maximum_fireballs: 1,
            fireball_amount: 0,
            coefficient_1: 0.250,
            coefficient_2: 0.0,
        })
        .build();
}

pub fn create_boss(world: &mut World) {
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
            projectile_sprite: "bossfireball".to_string(),
            maximum_fireballs: 2,
            fireball_amount: 0,
            coefficient_1: 0.075,
            coefficient_2: -0.05,
        })
        .build();
}

pub fn create_fireball_shower(world: &mut World) {
    let mut rng = thread_rng();
    let x_init: i32 = rng.gen_range(0, 100);
    let x_end: i32 = rng.gen_range(810, 900);
    let step: usize = rng.gen_range(90, 120);
    for x in (x_init..x_end).step_by(step) {
        world
            .create_entity()
            .with(Fireball { owner_id: None })
            .with(CalculateOutOfBounds)
            .with(Render {
                sprite: "fogo".to_string(),
                bounding_box: None,
            })
            .with(Position {
                position: Vector::new(x as f32, -100.0),
            })
            .with(Velocity {
                velocity: Vector::new(0.0, 250.0 + rng.gen_range(-10.0, 10.0)),
            })
            .build();
    }
}
