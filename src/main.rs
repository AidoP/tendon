mod fb;
mod maths;

use maths::*;
use fb::{Sampler, Texture};

fn main() {
    let mut fb = fb::Framebuffer::new().unwrap();
    let capsicum_texture = Texture::load("capsicum.png").unwrap();
    let capsicum_sampler = Sampler { texture: &capsicum_texture };
    // Mapping
    fb.draw_tri(
        Tri([
            Vector2 { x: 10.0, y: 10.0 },
            Vector2 { x: 10.0, y: 60.0 },
            Vector2 { x: 60.0, y: 10.0 },
        ]),
        Tri([
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 0.0, y: 1.0 },
            Vector2 { x: 1.0, y: 0.0 },
        ]),
        &capsicum_sampler
    );

    // Tiling
    fb.draw_tri(
        Tri([
            Vector2 { x: 70.0, y: 10.0 },
            Vector2 { x: 170.0, y: 10.0 },
            Vector2 { x: 70.0, y: 60.0 },
        ]),
        Tri([
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 2.0, y: 0.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ]),
        &capsicum_sampler
    );
    fb.draw_tri(
        Tri([
            Vector2 { x: 170.0, y: 10.0 },
            Vector2 { x: 170.0, y: 60.0 },
            Vector2 { x: 70.0, y: 60.0 },
        ]),
        Tri([
            Vector2 { x: 2.0, y: 0.0 },
            Vector2 { x: 2.0, y: 1.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ]),
        &capsicum_sampler
    );

    // Warping
    fb.draw_tri(
        Tri([
            Vector2 { x: 250.0, y: 10.0 },
            Vector2 { x: 350.0, y: 10.0 },
            Vector2 { x: 180.0, y: 60.0 },
        ]),
        Tri([
            Vector2 { x: 0.0, y: 0.0 },
            Vector2 { x: 2.0, y: 0.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ]),
        &capsicum_sampler
    );
    fb.draw_tri(
        Tri([
            Vector2 { x: 350.0, y: 10.0 },
            Vector2 { x: 280.0, y: 60.0 },
            Vector2 { x: 180.0, y: 60.0 },
        ]),
        Tri([
            Vector2 { x: 2.0, y: 0.0 },
            Vector2 { x: 2.0, y: 1.0 },
            Vector2 { x: 0.0, y: 1.0 },
        ]),
        &capsicum_sampler
    );

    fb.dump("screenshot.png").unwrap();
}
