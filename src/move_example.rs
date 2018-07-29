use ggez::*;
use ggez::graphics::{DrawMode,Point2,Rect,Color};
use node::*;
use pathfinder::*;
use std::borrow::BorrowMut;
use std::time::Instant;

static SIZE_COEFF : i32 = 5;

pub struct MainState {
    from: Vec<Point>,
    to: Vec<Point>,
    shapes : Vec<RectangleFieldShape>,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let dim = Dimension {width: 100, height: 100};
        let mut from : Vec<Point> = Vec::new();
        let mut to : Vec<Point> = Vec::new();
        let mut shapes : Vec<RectangleFieldShape> = Vec::new();

        shapes.push(RectangleFieldShape::new(10, 10, 10, 10, false));
        shapes.push(RectangleFieldShape::new(40, 20, 20, 20, false));
        shapes.push(RectangleFieldShape::new(40, 60, 20, 20, false));
        shapes.push(RectangleFieldShape::new(80, 80, 10, 10, false));
        
        for i in 0..49 {
            from.push(Point::new(0, 50-i));
            to.push(Point::new(90, 99 -i));
            from.push(Point::new(90, 99 -i));
            to.push(Point::new(0, 50-i));
        }

        let s = MainState { from: from, to: to, shapes: shapes };

        Ok(s)
    }

}

impl MainState {

    fn to_screen(&self, v: i32) -> f32 {
        return (v * SIZE_COEFF) as f32;
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut froms : Vec<Point> = Vec::new();
        let mut tos : Vec<Point> = Vec::new();

        while !self.from.is_empty() {
            let from = self.from.pop().unwrap();
            let to = self.to.pop().unwrap();

            if from.eq(&to) {
                froms.push(from);
                tos.push(to);
                continue;
            }

            let mut shapes : Vec<Box<FieldShape>> = Vec::new();

            for shape in &self.shapes {
                shapes.push(Box::new(shape.clone()));
            }

            let dim = Dimension {width: 100, height: 100};

            let field = PathField::new(shapes, dim);

            let pf = AStarPathFinder {field: field, from : from.clone(), to : to.clone()};

            //let start = Instant::now();
        
            let mut path = pf.get_path();

            //let (start, duration) = (Instant::now(), start.elapsed());
            //println!("Time elapsed : {:?}", duration);
        
            if path.is_empty() {
                froms.push(from);
            } else {
                froms.push(path.pop().unwrap());
            }
            tos.push(to);
        }

        self.from = froms;
        self.to = tos;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let shapes = &self.shapes;

        for shape in shapes {
            let ref s = *shape;
            graphics::set_color(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
            graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                    self.to_screen(s.point.x), 
                    self.to_screen(s.point.y), 
                    self.to_screen(s.width), 
                    self.to_screen(s.height)
                ))?;
        }

        graphics::set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;

        for point in &self.from {
            graphics::points(ctx,
                &[Point2::new(self.to_screen(point.x), self.to_screen(point.y))],
                SIZE_COEFF as f32)?;
        }
        graphics::present(ctx);
        Ok(())
    }
}