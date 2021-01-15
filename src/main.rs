mod fb;
mod maths;

use maths::*;
use fb::Colour;

fn main() {
    let mut fb = fb::Framebuffer::new().unwrap();
    fb.draw_tri(Tri([
        Vector2 { x: 150.0, y: 50.0 },
        Vector2 { x: 75.0, y: 100.0 },
        Vector2 { x: 200.0, y: 130.0 },
    ]));

    fb.dump("image.png");
}
