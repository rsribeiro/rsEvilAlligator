use specs::{Builder, Entity, World};

use crate::component::{CalculateOutOfBounds, Healing, Position, Render, Velocity};

use quicksilver::geom::{Rectangle, Shape, Vector};

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

// pub struct Healing {
//     atlas: Rc<RefCell<Asset<Atlas>>>,
//     position: Vector,
// }

// impl Healing {
//     pub fn new(atlas: Rc<RefCell<Asset<Atlas>>>) -> Result<Healing> {
//         let x = 50. + rand::random::<f32>() * 700.;
//         let position = Vector::new(x, -100.);
//         Ok(Healing { atlas, position })
//     }
// }

// impl Entity for Healing {
//     fn draw(&mut self, window: &mut Window) -> Result<()> {
//         let pos = self.position;
//         self.atlas.borrow_mut().execute(|loaded_atlas| {
//             let image = loaded_atlas.get("potion").unwrap().unwrap_image();
//             window.draw(&image.area().with_center(pos), Img(&image));
//             Ok(())
//         })
//     }

//     fn update(&mut self, window: &mut Window, _hero: &mut Hero) -> Result<()> {
//         let interval: f32 = window.update_rate() as f32;
//         self.position += Vector::new(0., 0.25 * interval);
//         Ok(())
//     }

//     fn collision(&mut self, hero: &mut Hero) -> Result<bool> {
//         let self_pos = self.position;
//         let hero_pos = hero.get_position();
//         let mut overlaps: bool = false;
//         self.atlas.borrow_mut().execute(|loaded_atlas| {
//             let potion_area = loaded_atlas
//                 .get("potion")
//                 .unwrap()
//                 .unwrap_image()
//                 .area()
//                 .with_center(self_pos);
//             let hero_area = loaded_atlas
//                 .get("heroi")
//                 .unwrap()
//                 .unwrap_image()
//                 .area()
//                 .with_center(*hero_pos);
//             overlaps = potion_area.overlaps(&hero_area);
//             Ok(())
//         })?;
//         if overlaps {
//             hero.give_life();
//             hero.give_score(50);
//             Ok(true)
//         } else {
//             Ok(false)
//         }
//     }

//     fn is_out_of_bounds(&self) -> bool {
//         self.position.y > 700.
//     }
// }
