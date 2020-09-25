extern crate ggez;
mod base;
mod pathfinder;
mod path_example;
mod move_example;

use ggez::*;
use base::*;
use pathfinder::*;
use std::env;
use std::path;
use std::process;
use ggez::conf::FullscreenType;

pub fn main() -> GameResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "move" {
        run("Move example", &mut move_example::MainState::new())
    } else if args[1] == "path" {
        run("Path example", &mut path_example::MainState::new())
    } else if args[1] == "test" {
        test()
    } else {
        println!("Unknown argument {}", args[1]);
        println!("Valid arguments are move, path or test", );
        process::exit(1);
    }

}

fn run<S>(title: &'static str, state: &mut S) -> Result<(), GameError>
where
    S: event::EventHandler,
{
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("../resources")
    };

    let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::True);

    /*let window_mode = conf::WindowMode::default().fullscreen_type(FullscreenType::Windowed)
        .dimensions(300.0, 300.0);
     */

    let cb = ContextBuilder::new("pathfinder-rust", "enricobn")
        .window_setup(
            conf::WindowSetup::default()
                .title(title)
                .vsync(false)
        )
        .window_mode(window_mode)
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("Drawable size {:?}", graphics::drawable_size(ctx));
    println!("Screen coordinates {:?}", graphics::screen_coordinates(ctx));

    event::run(ctx, events_loop, state)
}

fn test() -> Result<(), GameError> {
    let dim = Dimension {width: 100, height: 100};
    let mut shapes : Vec<Box<dyn FieldShape>> = Vec::new();

    shapes.push(Box::new(RectangleFieldShape {point: Point {x:0, y:0}, width:10, height:10}));
    shapes.push(Box::new(RectangleFieldShape {point: Point {x:20, y:20}, width:10, height:10}));

    let field = PathField::new(shapes, dim);
    
    println!("occupied {}", field.occupied(Point {x:5, y:5}));
    println!("occupied {}", field.occupied(Point {x:15, y:15}));

    let pf = AStarPathFinder {field, from : Point {x: 0, y:15}, to : Point {x: 30, y:25}};

    let path = pf.get_path();

    for point in path.iter() {
        println!("path {}", point);
    }

    Ok(())

}