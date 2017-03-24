extern crate glium;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::process;

use glium::DisplayBuild;
use glium::Surface;

fn main() {
    env_logger::init().unwrap();

    let file = env::args().nth(1);

    if file.is_none() {
        println!("Usage: gltf-viewer <path>");
        println!("  path:   The path to the glTF file");
        process::exit(1);
    }

    let file = file.unwrap();

    info!("Opening {}", file);

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("glTF Viewer"))
        .build_glium()
        .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
