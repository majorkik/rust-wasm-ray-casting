#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

use wasm4::*;

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);
}