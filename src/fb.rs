use std::ops::{Index, IndexMut, Deref, DerefMut};

#[link(name = "fb", kind = "static")]
extern "C" {
    fn fb_create() -> Framebuffer;
    fn fb_destroy(this: *mut Framebuffer);
}

/// Provides access to the linux framebuffer device.
/// Requires that the user is in the `video` group.
/// ```rust
/// use tendon::*;
/// let fb = Framebuffer::new().expect("Unable to create framebuffer");
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
    line_length: u32
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
    pub fn set(&mut self, x: usize, y: usize, colour: Colour) {
        let pos = (x + self.x_offset as usize) + (y + self.y_offset as usize) * self.line_length as usize;
        unsafe {*self.buffer.add(pos) = *colour }
    }
}
impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { fb_destroy(self) }
    }
}