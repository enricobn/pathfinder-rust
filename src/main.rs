extern crate ggez;
mod node;
mod pathfinder;

use ggez::*;
use ggez::graphics::{DrawMode, Point2};
use node::*;
use pathfinder::*;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
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

pub fn main() {
    test_path_field();
    
    /*
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
    */
}

fn test_path_field() {
    let dim = Dimension {width: 100, height: 100};

    let mut shapes : Vec<Box<FieldShape>> = Vec::new();

    shapes.push(Box::new(RectangleFieldShape {point: Point {x:0, y:0}, width:10, height:10, moving: false}));
    shapes.push(Box::new(RectangleFieldShape {point: Point {x:20, y:20}, width:10, height:10, moving: false}));

    let field = PathField::new(shapes, dim);
    
    println!("occupied {}", field.occupied(Point {x:5, y:5}));
    println!("occupied {}", field.occupied(Point {x:15, y:15}));

    let pf = AStarPathFinder {field: field, from : Point {x: 0, y:15}, to : Point {x: 30, y:25}};

    let path = pf.get_path();

    for point in path.iter() {
        println!("path {}", point);
    }

}