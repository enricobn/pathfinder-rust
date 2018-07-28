extern crate ggez;
mod node;
mod pathfinder;
mod circle_example;

use ggez::*;
use ggez::graphics::{DrawMode, Point2};
use node::*;
use pathfinder::*;

pub fn main() {
    test_path_field();
    
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut circle_example::MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
    
}

static SIZE_COEFF : i32 = 5;

fn display() {
    let dim = Dimension {width: 100, height: 100};
    let mut shapes : Vec<Box<FieldShape>> = Vec::new();

    shapes.push(Box::new(RectangleFieldShape::new(10, 10, 10, 10, false)));
    shapes.push(Box::new(RectangleFieldShape {point: Point::new(20, 20), width:10, height:10, moving: false} ));
    shapes.push(Box::new(RectangleFieldShape {point: Point::new(40, 20), width:20, height:20, moving: false} ));
    shapes.push(Box::new(RectangleFieldShape {point: Point::new(40, 60), width:20, height:20, moving: false} ));
    shapes.push(Box::new(RectangleFieldShape {point: Point::new(75, 75), width:10, height:10, moving: false} ));

    let field = PathField::new(shapes, dim);

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