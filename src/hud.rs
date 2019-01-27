use quicksilver::{
    geom::Shape,
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct HUD {
    font: Asset<Font>,
    draw_counter: u32,
    current_fps: f64,
}

const FPS_UPDATE_CYCLES: u32 = 10;

impl HUD {
    pub fn new() -> Result<HUD> {
        let font = Asset::new(Font::load("cmunrm.ttf"));
        Ok(HUD {
            font,
            draw_counter: 0,
            current_fps: 60.,
        })
    }

    pub fn draw(&mut self, window: &mut Window, lives: i32, score: i32) -> Result<()> {
        self.draw_counter += 1;
        if self.draw_counter % FPS_UPDATE_CYCLES == 0 {
            self.draw_counter = 0;
            self.current_fps = window.current_fps();
        }

        let current_fps = self.current_fps;
        self.font.execute(|font| {
            let style = FontStyle::new(48.0, Color::BLACK);

            let fps = font.render(&format!("{:.0}", current_fps), &style)?;
            let lives = font.render(&format!("{}", lives), &style)?;
            let score = font.render(&format!("{}", score), &style)?;

            window.draw(&fps.area().with_center((20, 587)), Img(&fps));
            window.draw(&lives.area().with_center((10, 20)), Img(&lives));
            window.draw(&score.area().with_center((730, 20)), Img(&score));
            Ok(())
        })
    }
}
