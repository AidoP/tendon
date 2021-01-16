mod fb;
mod maths;

use maths::*;
use fb::Colour;

fn main() {
    let mut fb = fb::Framebuffer::new().unwrap();
    fb.draw_tri(
        Tri([
            Vector2 { x: 100.0, y: 600.0 },
            Vector2 { x: 1100.0, y: 600.0 },
            Vector2 { x: 600.0, y: 100.0 },
        ]),
        [
            Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        ]
    );

    fb.dump("screenshot.png").unwrap();
}
