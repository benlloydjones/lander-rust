use geo::{intersects::Intersects, Coordinate, LineString, Polygon};
use wasm_bindgen::prelude::*;

use super::gravity::Gravity;
use super::input_buffer::InputBuffer;
use super::terrain::Terrain;
use super::thrusters::{NetThrust, Thrusters};

#[wasm_bindgen]
pub struct Lander {
    x: f64,
    y: f64,
    width: i32,
    height: i32,
    speed_x: f64,
    speed_y: f64,
    thrusters: Thrusters,
    gravity: Gravity,
}

impl Lander {
    pub fn new(gravity: Gravity, thrusters: Thrusters) -> Lander {
        Lander {
            x: 315.0,
            y: 10.0,
            width: 10,
            height: 10,
            speed_x: 0.0,
            speed_y: 0.0,
            thrusters,
            gravity,
        }
    }
    pub fn get_position(&self) -> Vec<Vec<i32>> {
        let x = self.x as i32;
        let y = self.y as i32;
        vec![
            vec![x, y],
            vec![x + self.width, y],
            vec![x + self.width, y + self.height],
            vec![x, y + self.height],
        ]
    }

    pub fn get_speed(&self) -> (f64, f64) {
        (self.speed_x, self.speed_y)
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.x < 0.0 || self.x > 630.0 || self.y < 0.0 || self.y > 470.0
    }

    fn get_net_thrust(&self, input: &InputBuffer) -> (f64, f64) {
        let thrust_x: f64;
        let thrust_y: f64;
        match self.thrusters.net_thrust(input) {
            NetThrust::Down => {
                thrust_x = 0.0;
                thrust_y = self.gravity.acceleration_due_to_gravity
                    - self.thrusters.acceleration_due_to_thrusters;
            }
            NetThrust::Left => {
                thrust_x = self.thrusters.acceleration_due_to_thrusters;
                thrust_y = self.gravity.acceleration_due_to_gravity;
            }
            NetThrust::Right => {
                thrust_x = -self.thrusters.acceleration_due_to_thrusters;
                thrust_y = self.gravity.acceleration_due_to_gravity;
            }
            NetThrust::DownLeft => {
                thrust_x = self.thrusters.acceleration_due_to_thrusters;
                thrust_y = self.gravity.acceleration_due_to_gravity
                    - self.thrusters.acceleration_due_to_thrusters;
            }
            NetThrust::DownRight => {
                thrust_x = -self.thrusters.acceleration_due_to_thrusters;
                thrust_y = self.gravity.acceleration_due_to_gravity
                    - self.thrusters.acceleration_due_to_thrusters;
            }
            NetThrust::None => {
                thrust_x = 0.0;
                thrust_y = self.gravity.acceleration_due_to_gravity;
            }
        }
        (thrust_x, thrust_y)
    }

    pub fn successfully_landed(&self, terrain: &Terrain) -> bool {
        if !self.intersects_terrain(&terrain) {
            return false;
        }
        let landing_zone = terrain.get_landing_zone_points();
        let landing_zone_start_x = landing_zone[0][0];
        let landing_zone_end_x = landing_zone[1][0];
        if self.x as i32 > landing_zone_start_x
            && (self.x as i32 + self.width) < landing_zone_end_x
            && self.speed_x > -1.0
            && self.speed_x < 1.0
            && self.speed_y < 5.0
        {
            return true;
        }
        false
    }

    pub fn intersects_terrain(&self, terrain: &Terrain) -> bool {
        let terrain_poly = create_poly(terrain.get_points());
        let lander_poly = create_poly(&self.get_position());
        terrain_poly.intersects(&lander_poly)
    }

    pub fn move_lander(mut self, elapsed_time_in_ms: u32, input: &InputBuffer) -> Lander {
        let (speed_x, speed_y) = self.get_speed();
        let (thrust_x, thrust_y) = self.get_net_thrust(input);
        self.speed_x = speed_x + (thrust_x * (elapsed_time_in_ms as f64 / 1000.0));
        self.speed_y = speed_y + (thrust_y * (elapsed_time_in_ms as f64 / 1000.0));
        let (speed_x, speed_y) = self.get_speed();
        self.x = self.x + (speed_x * (elapsed_time_in_ms as f64 / 1000.0));
        self.y = self.y + (speed_y * (elapsed_time_in_ms as f64 / 1000.0));
        self
    }
}

fn generate_coord_seq(coords: &Vec<Vec<i32>>) -> Vec<Coordinate<i32>> {
    let mut coordinates: Vec<Coordinate<i32>> = vec![];

    for coord in coords {
        coordinates.push(Coordinate {
            x: coord[0],
            y: coord[1],
        })
    }
    coordinates
}

fn create_poly(raw_coords: &Vec<Vec<i32>>) -> Polygon<i32> {
    let coords = generate_coord_seq(raw_coords);
    let line_string = LineString(coords);
    Polygon::new(line_string, vec![])
}
