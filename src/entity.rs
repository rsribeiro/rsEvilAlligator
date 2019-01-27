use crate::hero::Hero;

use quicksilver::{lifecycle::Window, Result};

pub trait Entity {
    fn draw(&mut self, window: &mut Window) -> Result<()>;
    fn update(&mut self, window: &mut Window, hero: &mut Hero) -> Result<()>;
    fn collision(&mut self, hero: &mut Hero) -> Result<bool>;
    fn is_out_of_bounds(&self) -> bool;
}
