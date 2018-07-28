use ggez::*;
use ggez::graphics::{DrawMode, Point2};

pub struct MainState {
    pos_x: f32,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, 380.0),
                         100.0,
                         2.0)?;
        graphics::present(ctx);
        Ok(())
    }
}
