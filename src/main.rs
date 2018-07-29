extern crate ggez;
mod node;
mod pathfinder;
mod path_example;
mod move_example;

use ggez::*;
use node::*;
use pathfinder::*;

pub fn main() {
    //test_path_field();
    run("Move example", Box::new(|ctx| move_example::MainState::new()));   
}

fn run<S>(title: &'static str, state: Box<Fn(&mut Context) -> S>) 
where
    S: event::EventHandler,
{
    let mut c = conf::Conf::new();
    c.window_setup = c.window_setup.title(title);

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let s = &mut state(ctx);
    event::run(ctx, s).unwrap();
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