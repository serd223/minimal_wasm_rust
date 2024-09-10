use std::mem;
use std::ptr::slice_from_raw_parts_mut;

use framebrush::{Canvas, Color};

pub fn rotate_x(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        x,
        (y * rot.cos()) - (z * rot.sin()),
        (y * rot.sin()) + (z * rot.cos()),
    )
}
pub fn rotate_y(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (z * rot.sin()) + (x * rot.cos()),
        y,
        (z * rot.cos()) - (x * rot.sin()),
    )
}
pub fn rotate_z(x: f32, y: f32, z: f32, rot: f32) -> (f32, f32, f32) {
    (
        (x * rot.cos()) - (y * rot.sin()),
        (x * rot.sin()) + (y * rot.cos()),
        z,
    )
}

const CUBE_VERTICES: [(f32, f32, f32); 8] = [
    (-1., -1., -1.),
    (1., -1., -1.),
    (-1., -1., 1.),
    (1., -1., 1.),
    (-1., 1., -1.),
    (1., 1., -1.),
    (-1., 1., 1.),
    (1., 1., 1.),
];

static mut ROT_X: f32 = 0.;
static mut ROT_Y: f32 = 0.;
static mut ROT_Z: f32 = 0.;

/// `0xaabbggrr`
///
/// Color are values are read from left to right as usual.
/// For example:
/// ABgr(0xff123456) == Rgba(0x563412ff)
struct ABgr(u32);

impl Color<u32> for ABgr {
    fn pixel(&self, _buf: &mut [u32], _index: usize) -> u32 {
        self.0
    }
}

const RED: ABgr = ABgr(0xff756ce0);
const BG: ABgr = ABgr(0xff342c28);

#[no_mangle]
pub extern "C" fn allocate_image(width: usize, height: usize) -> *mut u32 {
    let mut v: Vec<u32> = Vec::with_capacity(width * height);
    let ret = v.as_mut_ptr();
    mem::forget(v);
    ret
}

#[no_mangle]
pub extern "C" fn frame(image_ptr: *mut u32, width: usize, height: usize, delta: f32) {
    let image_data = slice_from_raw_parts_mut(image_ptr, width * height);

    unsafe {
        ROT_X += 1. * delta;
        ROT_Y += 5. * delta;
        ROT_Z += 3. * delta;
    }

    let cube_transform = unsafe {
        CUBE_VERTICES.map(|(x, y, z)| {
            let (x, y, z) = rotate_x(x, y, z, ROT_X);
            let (x, y, z) = rotate_y(x, y, z, ROT_Y);
            let (x, y, z) = rotate_z(x, y, z, ROT_Z);

            let (x, y, z) = ((x + 5.) * 20., (y + 5.) * 20., (z + 5.) * 20.);
            (x as usize, y as usize, z as usize)
        })
    };

    let mut canvas =
        unsafe { Canvas::new(&mut (*image_data), (width, height), (width / 2, height / 2)) };
    canvas.fill(BG.0);

    for (x0, y0, _) in cube_transform {
        for (x1, y1, _) in cube_transform {
            canvas.line(x0, y0, x1, y1, &RED);
        }
    }
}
