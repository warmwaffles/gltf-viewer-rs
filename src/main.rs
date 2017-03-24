#[macro_use]
extern crate log;

// #[macro_use]
// extern crate quick_error;

extern crate env_logger;
extern crate glium;
extern crate gltf;

use std::env;
use std::process;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;

use glium::DisplayBuild;
use glium::Surface;
use gltf::v1;

#[derive(Default)]
pub struct GltfAssets {
    pub gltf: Box<gltf::v1::Gltf>,
    pub files: HashMap<String, std::fs::File>,
}

pub fn load_gltf(path: &Path) -> Result<GltfAssets, gltf::v1::Error> {
    let gltf = gltf::v1::Gltf::open(path)?;

    let mut assets = GltfAssets {
        gltf: Box::new(gltf),
        .. Default::default()
    };

    // TODO: Load all buffers into asset pack
    // TODO: Load all shaders into shader pack

    for buffer in &assets.gltf.buffers {
        println!("{:?}", buffer);
    }

    Ok(assets)
}

fn main() {
    env_logger::init().unwrap();

    let arg = env::args().nth(1);

    if arg.is_none() {
        println!("Usage: gltf-viewer <path>");
        println!("  path:   The path to the glTF file");
        process::exit(1);
    }

    let arg = arg.unwrap();
    let path = Path::new(&arg);

    // info!("Opening {:?}", path);

    let asset = load_gltf(&path);

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
