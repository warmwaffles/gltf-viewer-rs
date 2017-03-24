extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

fn main() {
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
