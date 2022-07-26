use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputBuffer {
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

#[wasm_bindgen]
impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            left: false,
            right: false,
            down: false,
        }
    }
    pub fn receive_key_down(&mut self, key_name: String) {
        match &key_name[..] {
            "ArrowRight" => self.right = true,
            "ArrowLeft" => self.left = true,
            "ArrowDown" => self.down = true,
            _ => (),
        }
    }

    pub fn receive_key_up(&mut self, key_name: String) {
        match &key_name[..] {
            "ArrowRight" => self.right = false,
            "ArrowLeft" => self.left = false,
            "ArrowDown" => self.down = false,
            _ => (),
        }
    }
}
