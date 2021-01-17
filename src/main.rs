mod fb;
mod maths;

use maths::*;
use fb::{Sampler, Texture};

fn main() {
    let mut fb = fb::Framebuffer::new().unwrap();
    fb.dump("screenshot.png").unwrap();
}
