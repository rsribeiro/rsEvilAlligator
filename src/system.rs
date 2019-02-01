use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{Duration, Instant},
};

use crate::{
    component::{
        Boss, CalculateOutOfBounds, Enemy, Fireball, Healing, Hero, Label, Position, Render,
        Shooter, Velocity,
    },
    resources::{DeltaTime, KeyboardKeys, LabelVariable, PressedKeys, VariableDictionary},
};

use specs::{BitSet, Entities, Entity, Read, ReadStorage, System, WriteStorage};

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Atlas, Background::Img, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct RenderSystem<'a> {
    window: &'a mut Window,
    atlas: Rc<RefCell<Asset<Atlas>>>,
}

impl<'a> RenderSystem<'a> {
    pub fn new(
        window: &'a mut Window,
        atlas: Rc<RefCell<Asset<Atlas>>>,
    ) -> Result<RenderSystem<'a>> {
        Ok(RenderSystem { window, atlas })
    }

    fn render(&mut self, render: &mut Render, position: Vector) -> Result<()> {
        let window = &mut self.window;
        self.atlas.borrow_mut().execute(|loaded_atlas| {
            let image = loaded_atlas.get(&render.sprite).unwrap().unwrap_image();
            let area = image.area();
            render.bounding_box = Some(area);
            window.draw(&area.with_center(position), Img(&image));
            Ok(())
        })
    }
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Render>,
        ReadStorage<'a, Hero>,
    );

    fn run(&mut self, (pos, mut render, hero): Self::SystemData) {
        use specs::Join;

        for (pos, render, _) in (&pos, &mut render, !&hero).join() {
            self.render(render, pos.position).unwrap();
        }

        for (pos, render, hero) in (&pos, &mut render, &hero).join() {
            if hero.render {
                self.render(render, pos.position).unwrap();
            }
        }
    }
}

pub struct WalkSystem;

impl<'a> System<'a> for WalkSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        use specs::Join;
        let time_step =
            delta.duration.as_secs() as f32 + (delta.duration.subsec_nanos() as f32 * 1e-9);

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.position += vel.velocity * time_step;
        }
    }
}

pub struct LabelRenderSystem<'a> {
    window: &'a mut Window,
    font: Rc<RefCell<Asset<Font>>>,
}

impl<'a> LabelRenderSystem<'a> {
    pub fn new(window: &mut Window, font: Rc<RefCell<Asset<Font>>>) -> Result<LabelRenderSystem> {
        Ok(LabelRenderSystem { window, font })
    }
}

impl<'a> System<'a> for LabelRenderSystem<'a> {
    type SystemData = (
        Read<'a, VariableDictionary>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Label>,
    );

    fn run(&mut self, (dict, pos, render): Self::SystemData) {
        use specs::Join;

        for (pos, render) in (&pos, &render).join() {
            let window = &mut self.window;
            self.font
                .borrow_mut()
                .execute(|font| {
                    let rendered_label = font.render(
                        &dict.dictionary.get(&render.bind_variable).unwrap(),
                        &render.font_style,
                    )?;
                    window.draw(
                        &rendered_label.area().with_center(pos.position),
                        Img(&rendered_label),
                    );
                    Ok(())
                })
                .unwrap();
        }
    }
}

pub struct HeroControlSystem;

impl<'a> System<'a> for HeroControlSystem {
    type SystemData = (
        Read<'a, PressedKeys>,
        WriteStorage<'a, Hero>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (pressed_keys, mut hero, mut pos, mut vel): Self::SystemData) {
        use specs::Join;

        for (vel, pos, hero) in (&mut vel, &mut pos, &mut hero).join() {
            vel.velocity.y = if pos.position.y >= 425.0 {
                if pressed_keys
                    .pressed_keys
                    .contains(KeyboardKeys::KeyUp as u32)
                {
                    -400.0
                } else {
                    0.0
                }
            } else if pos.position.y <= 300.0 {
                200.0
            } else {
                vel.velocity.y
            };

            vel.velocity.x = if pressed_keys
                .pressed_keys
                .contains(KeyboardKeys::KeyRight as u32)
                && !pressed_keys
                    .pressed_keys
                    .contains(KeyboardKeys::KeyLeft as u32)
            {
                250.0
            } else if !pressed_keys
                .pressed_keys
                .contains(KeyboardKeys::KeyRight as u32)
                && pressed_keys
                    .pressed_keys
                    .contains(KeyboardKeys::KeyLeft as u32)
            {
                -250.0
            } else {
                0.0
            };

            if hero.reset_position {
                pos.position = Vector::new(15.0, 300.0);
                hero.reset_position = false;
            }
        }
    }
}

pub struct OutOfBoundsSystem;

impl<'a> System<'a> for OutOfBoundsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Hero>,
        ReadStorage<'a, CalculateOutOfBounds>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (entities, hero, oob, mut pos): Self::SystemData) {
        use specs::Join;

        for (_, pos, _, _) in (&entities, &mut pos, &oob, &hero).join() {
            pos.position.x = if pos.position.x < 15.0 {
                15.0
            } else if pos.position.x > 785.0 {
                785.0
            } else {
                pos.position.x
            }
        }
        for (e, pos, _, _) in (&entities, &mut pos, &oob, !&hero).join() {
            if pos.position.y > 700.0 || pos.position.x < -100.0 || pos.position.x > 900.0 {
                entities.delete(e).unwrap();
            }
        }
    }
}

pub struct CollisionSystem;

impl CollisionSystem {
    fn hero_enemy_collision(
        hero: &mut Hero,
        enemy: &Enemy,
        hero_render: &Render,
        enemy_render: &Render,
        hero_pos: Vector,
        enemy_pos: Vector,
        entities: &Entities,
        e: Entity,
    ) -> bool {
        if hero_render.bounding_box.is_some() && enemy_render.bounding_box.is_some() {
            let (hero_body_area, hero_feet_area) =
                crate::hero::get_hero_body_feet_area(hero_render.bounding_box.unwrap(), hero_pos);
            let (enemy_head_area, enemy_body_area) = crate::enemy::get_enemy_head_body_area(
                enemy_render.bounding_box.unwrap(),
                enemy_pos,
            );

            if enemy_head_area.overlaps(&hero_feet_area) {
                hero.score += enemy.score;
                entities.delete(e).unwrap();
            } else if enemy_body_area.overlaps(&hero_body_area) {
                if !hero.blinking {
                    println!("porrada");
                    hero.lives -= 1;
                    hero.blinking = true;
                }
            }
        }
        hero.blinking
    }

    fn hero_healing_collision(
        hero: &mut Hero,
        healing: &Healing,
        hero_render: &Render,
        healing_render: &Render,
        hero_pos: Vector,
        healing_pos: Vector,
        entities: &Entities,
        e: Entity,
    ) {
        if hero_render.bounding_box.is_some() && healing_render.bounding_box.is_some() {
            let hero_bounding_box = hero_render.bounding_box.unwrap().with_center(hero_pos);
            let healing_bounding_box = healing_render
                .bounding_box
                .unwrap()
                .with_center(healing_pos);

            if hero_bounding_box.overlaps(&healing_bounding_box) {
                hero.lives += 1;
                hero.score += healing.score;
                entities.delete(e).unwrap();
            }
        }
    }

    fn hero_boss_collision(
        hero: &mut Hero,
        enemy: &Enemy,
        boss: &mut Boss,
        hero_render: &Render,
        enemy_render: &Render,
        hero_pos: Vector,
        enemy_pos: Vector,
        entities: &Entities,
        e: Entity,
    ) -> bool {
        if hero_render.bounding_box.is_some() && enemy_render.bounding_box.is_some() {
            let (hero_body_area, hero_feet_area) =
                crate::hero::get_hero_body_feet_area(hero_render.bounding_box.unwrap(), hero_pos);
            let (enemy_head_area, enemy_body_area) = crate::enemy::get_enemy_head_body_area(
                enemy_render.bounding_box.unwrap(),
                enemy_pos,
            );

            if enemy_head_area.overlaps(&hero_feet_area) {
                hero.score += enemy.score;
                hero.reset_position = true;
                hero.blinking = true;
                boss.lives -= 1;
                if boss.lives == 0 {
                    entities.delete(e).unwrap();
                }
            } else if enemy_body_area.overlaps(&hero_body_area) {
                if !hero.blinking {
                    println!("porrada");
                    hero.lives -= 1;
                    hero.blinking = true;
                }
            }
        }
        hero.blinking
    }
}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Hero>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Boss>,
        ReadStorage<'a, Healing>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Render>,
    );

    fn run(
        &mut self,
        (entities, mut hero, enemy, mut boss, healing, pos, render): Self::SystemData,
    ) {
        use specs::Join;

        for (e_hero, hero, hero_pos, hero_render) in (&entities, &mut hero, &pos, &render).join() {
            for (e, enemy_pos, enemy_render, enemy) in (&entities, &pos, &render, &enemy).join() {
                let boss: Option<&mut Boss> = boss.get_mut(e);
                if boss.is_none() {
                    if CollisionSystem::hero_enemy_collision(
                        hero,
                        enemy,
                        hero_render,
                        enemy_render,
                        hero_pos.position,
                        enemy_pos.position,
                        &entities,
                        e,
                    ) {
                        break;
                    }
                } else if boss.is_some() {
                    if CollisionSystem::hero_boss_collision(
                        hero,
                        enemy,
                        boss.unwrap(),
                        hero_render,
                        enemy_render,
                        hero_pos.position,
                        enemy_pos.position,
                        &entities,
                        e,
                    ) {
                        break;
                    }
                }
            }

            for (e, healing_pos, healing_render, healing) in
                (&entities, &pos, &render, &healing).join()
            {
                CollisionSystem::hero_healing_collision(
                    hero,
                    healing,
                    hero_render,
                    healing_render,
                    hero_pos.position,
                    healing_pos.position,
                    &entities,
                    e,
                );
            }

            if hero.lives == 0 {
                entities.delete(e_hero).unwrap();
            }
        }
    }
}

pub struct HeroBlinkingSystem;

impl<'a> System<'a> for HeroBlinkingSystem {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Hero>);

    fn run(&mut self, (delta_time, mut hero): Self::SystemData) {
        use specs::Join;

        for hero in (&mut hero).join() {
            if hero.blinking {
                hero.blink_timer += delta_time.duration;

                let blinking_time_sec = hero.blink_timer.as_secs() as f64
                    + (hero.blink_timer.subsec_nanos() as f64 * 1e-9);
                if blinking_time_sec > 1.25 {
                    hero.blink_timer = Duration::from_millis(0);
                    hero.blinking = false;
                    hero.render = true;
                } else {
                    hero.render = !hero.blinking || (blinking_time_sec / 0.15) as i32 % 2 == 0;
                }
            } else {
                hero.blink_timer = Duration::from_millis(0);
                hero.render = true;
            };
        }
    }
}

pub struct FireballSystem;

impl<'a> System<'a> for FireballSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Shooter>,
        ReadStorage<'a, Fireball>,
    );

    fn run(&mut self, (entities, mut position, shooter, fireball): Self::SystemData) {
        use specs::Join;

        // for shooter in (&shooter).join() {
        //     let mut fireball_count = 0;
        //     for fireball in (&fireball).join() {

        //     }
        // }
    }
}
