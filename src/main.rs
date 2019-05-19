extern crate eangine;
extern crate rand;
extern crate specs;

use rand::{thread_rng, Rng};

use eangine::{
    enemy::FireballShowerConfig,
    healing::HealingConfig,
    scene::{Scene, SceneConfig},
};

use specs::World;

use quicksilver::{
    geom::Vector,
    lifecycle::{run_with, run, Settings},
};

fn main() {
    let entity_factory = |world: &mut World, cycle_counter: u32| {
        let n: u32 = thread_rng().gen_range(0, 4);
        match n {
            0 => eangine::enemy::create_walker(world),
            1 => eangine::enemy::create_shooter(world),
            2 => eangine::enemy::create_flyer(world),
            3 => eangine::enemy::create_fireball_shower(world, FireballShowerConfig::default()),
            _ => {}
        }
        if cycle_counter % 3 == 0 {
            eangine::healing::create_healing_potion(world, HealingConfig::default());
        }
        Ok(())
    };

    let mut scene_config = SceneConfig {
        boss_cycle: 20,
        new_body_cycle: 125,
        ..SceneConfig::default()
    };
    scene_config.boss_config.shooter_config.projectile_sprite = "bossfireball".to_string();
    scene_config.boss_config.shooter_config.maximum_projectiles = 3;
    scene_config.boss_config.lives = 15;
    scene_config.boss_config.normal_lives = 8;

    run_with(
        "Evil Alligator",
        Vector::new(800, 600),
        Settings {
            icon_path: Some("icone.png"),
            show_cursor: false,
            ..Settings::default()
        },
        || Scene::new(Box::new(entity_factory), scene_config),
    );

    // run classic
    // run::<Scene>(
    //     "Evil Alligator",
    //     Vector::new(800, 600),
    //     Settings {
    //         icon_path: Some("icone.png"),
    //         show_cursor: false,
    //         ..Settings::default()
    //     }
    // );
}
