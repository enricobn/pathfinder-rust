extern crate ggez;
mod base;
mod pathfinder;
mod path_example;
mod move_example;

use ggez::*;
use base::*;
use pathfinder::*;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "move" {
        run("Move example", &mut move_example::MainState::new());
    } else if args[1] == "path" {
        run("Path example", &mut path_example::MainState::new());
    } else if args[1] == "test" {
        test();
    } else {
        println!("Unknown argument {}", args[1]);
        println!("Valid arguments are move, path or test", );
    }
}

fn run<S>(title: &'static str, state: &mut S) 
where
    S: event::EventHandler,
{
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title(title);
    c.window_mode.vsync = false;

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    event::run(ctx, state).unwrap();
}

fn test() {
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