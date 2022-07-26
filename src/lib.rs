extern crate console_error_panic_hook;
use std::panic;

mod gravity;
mod input_buffer;
mod lander;
mod terrain;
mod thrusters;

use wasm_bindgen::prelude::*;

use gravity::Gravity;
use input_buffer::InputBuffer;
use lander::Lander;
use terrain::Terrain;
use thrusters::Thrusters;

#[wasm_bindgen]
pub fn init_panic() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn get_lander() -> Lander {
    Lander::new(Gravity::new(1.6), Thrusters::new(4.0))
}

#[wasm_bindgen]
pub fn get_terrain(n_of_peaks: i32) -> Terrain {
    Terrain::new(n_of_peaks)
}

#[wasm_bindgen]
pub fn get_input_buffer() -> InputBuffer {
    InputBuffer::new()
}

#[wasm_bindgen]
pub fn lander_move(lander: Lander, input: &InputBuffer, elapsed_time_in_ms: u32) -> Lander {
    lander.move_lander(elapsed_time_in_ms, input)
}

#[wasm_bindgen]
pub fn lander_intersects_terrain(lander: &Lander, terrain: &Terrain) -> bool {
    lander.intersects_terrain(terrain)
}

#[wasm_bindgen]
pub fn lander_successfully_landed(lander: &Lander, terrain: &Terrain) -> bool {
    lander.successfully_landed(terrain)
}

#[wasm_bindgen]
pub fn lander_is_out_of_bounds(lander: &Lander) -> bool {
    lander.is_out_of_bounds()
}

#[wasm_bindgen]
pub fn lander_get_speed(lander: &Lander) -> Vec<f64> {
    let (speed_x, speed_y) = lander.get_speed();
    vec![speed_x, speed_y]
}

#[wasm_bindgen]
pub fn lander_get_coords(lander: &Lander) -> Vec<i32> {
    let position = lander.get_position();
    flatten_vector(position)
}

#[wasm_bindgen]
pub fn terrain_get_coords(terrain: &Terrain) -> Vec<i32> {
    let points = terrain.get_points();
    flatten_vector(points.to_vec())
}

#[wasm_bindgen]
pub fn terrain_get_landing_zone_coords(terrain: &Terrain) -> Vec<i32> {
    let coords = terrain.get_landing_zone_points();
    flatten_vector(coords.to_vec())
}

fn flatten_vector<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}
