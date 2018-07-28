use ggez::*;
use ggez::graphics::{DrawMode,Point2,Rect};
use node::*;
use pathfinder::*;
use std::rc::Rc;
use std;

static SIZE_COEFF : i32 = 5;

pub struct MainState {
    x: f32,
    y: f32,
    path: Vec<Point>,
    shapes : Vec<Box<RectangleFieldShape>>,
    i: i32
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let dim = Dimension {width: 100, height: 100};
        let mut shapes : Vec<Box<FieldShape>> = Vec::new();
        let mut shapes_cp : Vec<Box<RectangleFieldShape>> = Vec::new();

        /*shapes.push(Box::new(RectangleFieldShape::new(10, 10, 10, 10, false)));
        shapes_cp.push(Box::new(RectangleFieldShape::new(10, 10, 10, 10, false)));
        shapes.push(Box::new(RectangleFieldShape::new(20, 20, 10, 10, false)));
        shapes_cp.push(Box::new(RectangleFieldShape::new(20, 20, 10, 10, false)));
        shapes.push(Box::new(RectangleFieldShape::new(40, 20, 40, 40, false)));
        shapes_cp.push(Box::new(RectangleFieldShape::new(20, 20, 10, 10, false)));
        shapes.push(Box::new(RectangleFieldShape::new(40, 60, 40, 40, false)));
        shapes_cp.push(Box::new(RectangleFieldShape::new(40, 60, 40, 40, false)));
        shapes.push(Box::new(RectangleFieldShape::new(75, 75, 10, 10, false)));
        shapes_cp.push(Box::new(RectangleFieldShape::new(75, 75, 10, 10, false)));
        */

        let (shapes, shapes_cp) = MainState::add_shape(shapes, shapes_cp, RectangleFieldShape::new(10, 10, 10, 10, false));
        let (shapes, shapes_cp) = MainState::add_shape(shapes, shapes_cp, RectangleFieldShape::new(40, 20, 20, 20, false));
        let (shapes, shapes_cp) = MainState::add_shape(shapes, shapes_cp, RectangleFieldShape::new(40, 60, 20, 20, false));
        let (shapes, shapes_cp) = MainState::add_shape(shapes, shapes_cp, RectangleFieldShape::new(80, 80, 10, 10, false));
        
        let field = PathField::new(shapes, dim);

        let from = Point::new(0, 0);
        
        let to = Point::new(99, 99);

        let pf = AStarPathFinder {field: field, from : from, to : to};

        let path = pf.get_path();

        let s = MainState { x: 0.0, y: 0.0, path: path, shapes: shapes_cp, i:0 };

        Ok(s)
    }

    fn add_shape(mut shapes : Vec<Box<FieldShape>>, mut shapes_cp : Vec<Box<RectangleFieldShape>>, shape: RectangleFieldShape) -> (Vec<Box<FieldShape>>, Vec<Box<RectangleFieldShape>>) {
        shapes.push(Box::new(shape));
        shapes_cp.push(Box::new(shape.clone()));
        return (shapes, shapes_cp);
    }
}

impl MainState {

    fn to_screen(&self, v: i32) -> f32 {
        return (v * SIZE_COEFF) as f32;
    }

}

impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let last = self.path.pop();
        if last.is_some() {
            self.x = self.to_screen(last.unwrap().x);
            self.y = self.to_screen(last.unwrap().y);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //graphics::clear(ctx);

        let shapes = &self.shapes;

        for shape in shapes {
            let ref s = *shape;
            graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                    self.to_screen(s.point.x), 
                    self.to_screen(s.point.y), 
                    self.to_screen(s.width), 
                    self.to_screen(s.height)
                ))?;
        }

        graphics::points(ctx,
                         &[Point2::new(self.x, self.y)],
                         SIZE_COEFF as f32)?;
        graphics::present(ctx);
        Ok(())
    }
}
