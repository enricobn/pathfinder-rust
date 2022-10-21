use ggez::*;
use ggez::graphics::{DrawMode, Drawable, Rect, Color, Mesh, DrawParam};
use base::*;
use pathfinder::*;
use std::time::Instant;
use path_example::Point2;

/*
 * AMD Ryzen 5 3600 6-Core Processor
 * rustc 1.43.0
 * Time elapsed : 4.99 sec
 */
static SIZE_COEFF : i32 = 5;

pub struct MainState {
    pub title: &'static str,
    from: Vec<ColoredPoint>,
    to: Vec<Point>,
    shapes : Vec<RectangleFieldShape>,
    start: Option<Instant>,
    ended: bool,
    first: bool,
}

struct ColoredPoint {
    point: Point,
    color: Color,
}

impl MainState {

    pub fn new() -> MainState {
        let mut from : Vec<ColoredPoint> = Vec::new();
        let mut to : Vec<Point> = Vec::new();
        let mut shapes : Vec<RectangleFieldShape> = Vec::new();

        shapes.push(RectangleFieldShape::new(10, 10, 10, 10));
        shapes.push(RectangleFieldShape::new(40, 20, 20, 20));
        shapes.push(RectangleFieldShape::new(40, 60, 20, 20));
        shapes.push(RectangleFieldShape::new(75, 75, 10, 10));

        let green = Color::new(0.0, 1.0, 0.0, 1.0);
        let blue = Color::new(0.0, 0.0, 1.0, 1.0);

        for i in 0..49 {
            from.push(ColoredPoint { point: Point::new(0, 50 - i), color: green });
            to.push(Point::new(90, 99 - i));
            from.push(ColoredPoint { point: Point::new(90, 99 - i), color: blue });
            to.push(Point::new(0, 50 - i));
        }

        MainState { title: "Move example", from, to, shapes, start: None, ended: false, first : true }
    }

}

impl MainState {

    fn to_screen(&self, v: i32) -> f32 {
        return (v * SIZE_COEFF) as f32;
    }

}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // the first time, I want only to draw the initial position.
        if self.first {
            self.first = false;
            return Ok(());
        }

        // but I initialize the time on the first real update.
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }

        let mut froms : Vec<ColoredPoint> = Vec::new();
        let mut tos : Vec<Point> = Vec::new();

        let mut ended = true;

        while !self.from.is_empty() {
            let from = self.from.pop().unwrap();
            let to = self.to.pop().unwrap();

            if from.point.eq(&to) {
                froms.push(from);
                tos.push(to);
                continue;
            }

            ended = false;

            let mut shapes : Vec<Box<dyn FieldShape>> = Vec::new();

            for shape in &self.shapes {
                shapes.push(Box::new(shape.clone()));
            }

            for point in &self.from {
                shapes.push(Box::new(PointFieldShape {x: point.point.x, y: point.point.y}));
            }

            for point in &froms {
                shapes.push(Box::new(PointFieldShape {x: point.point.x, y: point.point.y}));
            }

            let dim = Dimension {width: 100, height: 100};

            let field = PathField::new(shapes, dim);

            let pf = AStarPathFinder {field, from : from.point.clone(), to : to.clone()};
        
            let mut path = pf.get_path();

            if path.is_empty() {
                froms.push(from);
            } else {
                froms.push(ColoredPoint { point: path.pop().unwrap(), color: from.color});
            }
            tos.push(to);
        }

        if !self.ended && ended {
            self.ended = true;
            let duration = self.start.unwrap().elapsed();
            println!("Time elapsed : {:?}", duration);
        }

        self.from = froms;
        self.to = tos;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        graphics::clear(ctx, black);

        // TODO use MeshBuilder?

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

        for point in &self.from {
            let rect = Rect::new(
                self.to_screen(point.point.x),
                self.to_screen(point.point.y),
                SIZE_COEFF as f32,
                SIZE_COEFF as f32
            );

            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, point.color)?;

            let param = DrawParam::new().dest(Point2::new(0.0, 0.0));

            mesh.draw(ctx, param)?;
        }

        graphics::present(ctx)
    }
}