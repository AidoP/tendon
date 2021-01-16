use std::{fs::File, io::{Read, Write}, path::Path};

use crate::maths::Tri;

#[link(name = "fb", kind = "static")]
extern "C" {
    fn fb_create() -> Framebuffer;
    fn fb_destroy(this: *mut Framebuffer);
}

/// Provides access to the linux framebuffer device.
/// Requires that the user is in the `video` group.
/// ```no_run
/// use tendon::*;
/// let fb = Framebuffer::new();
/// ```
#[repr(C)]
pub struct Framebuffer {
    buffer: *mut u32,
    buffer_len: usize,
    bytes_per_pixel: u32,
    red_offset: u32,
    green_offset: u32,
    blue_offset: u32,
    x_offset: u32,
    y_offset: u32,
    pub line_length: u32
}
impl Framebuffer {
    pub fn new() -> Option<Self> {
        let this = unsafe { fb_create() };
        if this.buffer.is_null() {
            None
        } else {
            assert_eq!(this.bytes_per_pixel, 4);
            Some(this)
        }
    }
    pub fn dump<P: AsRef<Path>>(&self, path: P) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(1366, 768);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let (r, g, b) = Colour::u32_to_rgb(self.get(x as _, y as _), self);
            *pixel = image::Rgb([r, g, b])
        }
        img.save(path)
    }
    pub fn get(&self, x: usize, y: usize) -> u32 {
        let pos = (x + self.x_offset as usize) + (y + self.y_offset as usize) * self.line_length as usize;
        if pos >= self.buffer_len {
            panic!("Pixel ({}, {}) is out of framebuffer bounds", x, y)
        }
        #[cfg(not(target_arch="x86_64"))]
        if pos + self.buffer as usize >= isize::MAX {
            panic!("Framebuffer cannot be indexed due to platform addressing constraints")
        }
        unsafe { *self.buffer.add(pos) }
    }
    pub fn set(&mut self, x: usize, y: usize, colour: Colour) {
        let pos = (x + self.x_offset as usize) + (y + self.y_offset as usize) * self.line_length as usize;
        if pos >= self.buffer_len {
            panic!("Pixel ({}, {}) is out of framebuffer bounds", x, y)
        }
        #[cfg(not(target_arch="x86_64"))]
        if pos + self.buffer as usize >= isize::MAX {
            panic!("Framebuffer cannot be indexed due to platform addressing constraints")
        }
        unsafe {*self.buffer.add(pos) = colour.convert(self) }
    }
    pub fn draw_tri<'a>(&mut self, tri: Tri, uvs: [crate::maths::Vector3; 3], /*sampler: &Sampler<'a>*/) {
        let mut a = 0;
        let mut b = 1;
        let mut c = 2;
        if tri[a].y > tri[b].y {
            std::mem::swap(&mut a, &mut b)
        }
        if tri[a].y > tri[c].y {
            std::mem::swap(&mut a, &mut c)
        }
        if tri[b].y > tri[c].y {
            std::mem::swap(&mut b, &mut c)
        }

        let high_edge = tri[c] - tri[a];
        let top_edge = tri[b] - tri[a];
        let bottom_edge = tri[c] - tri[b];
        let midpoint_x = tri[a].x + top_edge.y / high_edge.y * high_edge.x;
        let left_triangle = midpoint_x < tri[b].x;

        if tri[a].y as usize != tri[b].y as usize {
            let (l_grad, r_grad) = if left_triangle {
                (high_edge.inverse_gradient(), top_edge.inverse_gradient())
            } else {
                (top_edge.inverse_gradient(), high_edge.inverse_gradient())
            };
            let mut x_start = tri[a].x;
            let mut x_end = x_start;
            for y in tri[a].y as usize .. tri[b].y as usize {
                for x in x_start as usize .. x_end as usize {
                    let uv = tri.interpolate(&uvs, x as _, y as _);
                    let c = Colour(
                        ((uv.x * 255.0) as u32) << 24 |
                        ((uv.y * 255.0) as u32) << 16 |
                        ((uv.z * 255.0) as u32) << 8  |
                        0xFF
                    );
                    self.set(x, y, /*sampler.sample(uv.x, uv.y)*/ c)
                }
                x_start += l_grad;
                x_end += r_grad;
            }
        }

        if tri[b].y as usize != tri[c].y as usize {
            let mut x_start = tri[b].x;
            let mut x_end = midpoint_x;
            if left_triangle {
                std::mem::swap(&mut x_start, &mut x_end)
            }
            let (l_grad, r_grad) = if left_triangle {
                (high_edge.inverse_gradient(), bottom_edge.inverse_gradient())
            } else {
                (bottom_edge.inverse_gradient(), high_edge.inverse_gradient())
            };
            for y in tri[b].y as usize ..= tri[c].y as usize {
                for x in x_start as usize .. x_end as usize {
                    let uv = tri.interpolate(&uvs, x as _, y as _);
                    let c = Colour(
                        ((uv.x * 255.0) as u32) << 24 |
                        ((uv.y * 255.0) as u32) << 16 |
                        ((uv.z * 255.0) as u32) << 8 |
                        0xFF
                    );
                    self.set(x, y, /*sampler.sample(uv.x, uv.y)*/ c)
                }
                x_start += l_grad;
                x_end += r_grad;
            }
        }
    }
}
impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { fb_destroy(self) }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Colour(pub u32);
impl Colour {
    /// Create a colour with the correct byte order for the framebuffer
    pub fn convert(self, fb: &Framebuffer) -> u32 {
        (self.0 & 0xFF00_0000) >> 24 << fb.red_offset |
        (self.0 & 0x00FF_0000) >> 16 << fb.green_offset |
        (self.0 & 0x0000_FF00) >> 8 << fb.blue_offset
    }
    /// Create a colour with the correct byte order for the framebuffer
    pub fn u32_to_rgb(colour: u32, fb: &Framebuffer) -> (u8, u8, u8) {
        (
            (colour >> fb.red_offset & 0xFF) as u8,
            (colour >> fb.green_offset & 0xFF) as u8,
            (colour >> fb.blue_offset & 0xFF) as u8
        )
    }
}

pub struct Sampler<'a> {
    pub texture: &'a Texture
}
impl<'a> Sampler<'a> {
    /// Get the pixel nearest `x` and `y`
    pub fn sample(&self, x: f64, y: f64) -> Colour {
        self.texture.get(
            f64::floor(x.fract().abs() * self.texture.width as f64) as usize,
            f64::floor(y.fract().abs() * self.texture.height as f64) as usize
        )
    }
}
pub struct Texture {
    pub buffer: Vec<Colour>,
    pub width: usize,
    pub height: usize
}
impl Texture {
    pub fn get(&self, x: usize, y: usize) -> Colour {
        #[cfg(debug_assertions)]
        if x >= self.width {
            panic!("Cannot index texture of width {} by x index {}", self.width, x)
        }
        self.buffer[x + y * self.width]
    }
}