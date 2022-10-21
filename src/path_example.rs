use ggez::*;
use ggez::graphics::{DrawMode, Drawable, Rect, Color, Mesh, DrawParam};
use base::*;
use pathfinder::*;
use std::borrow::BorrowMut;
use std::time::Instant;

static SIZE_COEFF : i32 = 5;
pub type Point2 = nalgebra::Point2<f32>;

pub struct MainState {
    path: Vec<Point>,
    shapes : Vec<Box<RectangleFieldShape>>,
}

impl MainState {
    pub fn new() -> MainState {
        let dim = Dimension { width: 100, height: 100 };
        let mut shapes : Vec<Box<dyn FieldShape>> = Vec::new();
        let mut shapes_cp : Vec<Box<RectangleFieldShape>> = Vec::new();

        MainState::add_shape(shapes.borrow_mut(), shapes_cp.borrow_mut(),
                             RectangleFieldShape::new(10, 10, 10, 10));
        MainState::add_shape(shapes.borrow_mut(), shapes_cp.borrow_mut(),
                             RectangleFieldShape::new(40, 20, 20, 20));
        MainState::add_shape(shapes.borrow_mut(), shapes_cp.borrow_mut(),
                             RectangleFieldShape::new(40, 60, 20, 20));
        MainState::add_shape(shapes.borrow_mut(), shapes_cp.borrow_mut(),
                             RectangleFieldShape::new(80, 80, 10, 10));
        
        let field = PathField::new(shapes, dim);

        let from = Point::new(0, 0);
        
        let to = Point::new(99, 99);

        let pf = AStarPathFinder {field, from, to };

        let start = Instant::now();
        
        let path = pf.get_path();

        println!("Points: {}", path.len());

        let duration= start.elapsed();
        println!("Time elapsed : {:?}", duration);

        MainState { path, shapes: shapes_cp }
    }

    fn add_shape(shapes : &mut Vec<Box<dyn FieldShape>>, shapes_cp : &mut Vec<Box<RectangleFieldShape>>, shape: RectangleFieldShape) {
        shapes.push(Box::new(shape));
        shapes_cp.push(Box::new(shape));
    }
}

impl MainState {

    fn to_screen(&self, v: i32) -> f32 {
        return (v * SIZE_COEFF) as f32;
    }

}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        graphics::clear(ctx, black);

        let shapes = &self.shapes;

        for shape in shapes {
            let ref s = *shape;
            let color = Color::new(1.0, 0.0, 0.0, 1.0);

            let rect = Rect::new(
                self.to_screen(s.point.x),
                self.to_screen(s.point.y),
                self.to_screen(s.width),
                self.to_screen(s.height)
            );

            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;

            let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

            mesh.draw(ctx, param)?;
        }

        for point in self.path.iter() {
            let rect = Rect::new(
                0.,
                0.,
                SIZE_COEFF as f32,
                SIZE_COEFF as f32
            );

            let color = Color::new(1.0, 1.0, 1.0, 1.0);

            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;

            let param = DrawParam::new().dest(Point2::new(self.to_screen(point.x), self.to_screen(point.y)));

            mesh.draw(ctx, param)?;
        }

        graphics::present(ctx)
    }
}
