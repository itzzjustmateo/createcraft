#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = web::render_triangle().await {
            web_sys::console::error_1(&error);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Build the browser client for wasm32-unknown-unknown and serve it with Trunk.");
}
