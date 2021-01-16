#include <fcntl.h>
#include <unistd.h>
#include <linux/fb.h>
#include <sys/mman.h>
#include <sys/ioctl.h>
#include <stdio.h>

struct fb {
    unsigned* buffer;
    size_t buffer_len;
    unsigned bytes_per_pixel;
    unsigned red_offset;
    unsigned green_offset;
    unsigned blue_offset;
    unsigned x_offset;
    unsigned y_offset;
    unsigned x_res;
    unsigned y_res;
    unsigned line_length;
};

struct fb fb_create() {
    struct fb fb;
    int fb_file = open("/dev/fb0", O_RDWR);
    if (fb_file < 0) {
        fb.buffer = 0;
        return fb;
    }

    struct fb_fix_screeninfo fix_info;
    struct fb_var_screeninfo var_info;
    ioctl(fb_file, FBIOGET_FSCREENINFO, &fix_info);
    ioctl(fb_file, FBIOGET_VSCREENINFO, &var_info);


    var_info.bits_per_pixel = 32;
    var_info.grayscale = 0;
    if (ioctl(fb_file, FBIOPUT_VSCREENINFO, &var_info) < 0)
        // Failed to change to the desired value so reload
        ioctl(fb_file, FBIOGET_VSCREENINFO, &var_info);
    
    fb.bytes_per_pixel = var_info.bits_per_pixel / 8;
    fb.red_offset = var_info.red.offset;
    fb.green_offset = var_info.green.offset;
    fb.blue_offset = var_info.blue.offset;
    fb.x_offset = var_info.xoffset;
    fb.y_offset = var_info.yoffset;
    fb.x_res = var_info.xres;
    fb.y_res = var_info.yres;
    fb.line_length = fix_info.line_length / sizeof(unsigned);
    fb.buffer_len = fix_info.smem_len;

    void* error;
    fb.buffer = error = mmap(NULL, fb.buffer_len, PROT_WRITE, MAP_SHARED, fb_file, 0);
    if (error == MAP_FAILED)
        fb.buffer = 0;

    return fb;
}
void fb_destroy(struct fb* fb) {
    munmap(fb->buffer, fb->buffer_len);
}