use std::mem;
use std::ptr::slice_from_raw_parts_mut;

use framebrush::{Canvas, RGBu32};

#[no_mangle]
pub extern "C" fn allocate_image(width: usize, height: usize) -> *mut u32 {
    let mut v: Vec<u32> = Vec::with_capacity(width * height);
    let ret = v.as_mut_ptr();
    mem::forget(v);
    ret
}

#[no_mangle]
pub extern "C" fn frame(image_ptr: *mut u32, width: usize, height: usize) {
    let image_data = slice_from_raw_parts_mut(image_ptr, width * height);

    unsafe {
        let mut canvas = Canvas::new(&mut (*image_data), (width, height), (width, height));
        canvas.rect(10, 10, 50, 50, &RGBu32::Pixel(0xff00ff00)); // 0xaabbggrr
    }
}
