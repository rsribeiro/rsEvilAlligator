use specs::{Builder, Entity, World};

use crate::component::{CalculateOutOfBounds, Healing, Position, Render, Velocity};

use quicksilver::geom::Vector;

pub fn create_healing_potion(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(CalculateOutOfBounds)
        .with(Position {
            position: Vector::new(50.0 + rand::random::<f32>() * 700.0, -100.0),
        })
        .with(Velocity {
            velocity: Vector::new(0.0, 250.0),
        })
        .with(Render {
            sprite: "potion".to_string(),
            bounding_box: None,
        })
        .with(Healing { score: 50 })
        .build()
}
