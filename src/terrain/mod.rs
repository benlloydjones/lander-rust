use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Terrain {
    points: Vec<Vec<i32>>,
    landing_zone: Vec<Vec<i32>>,
}

impl Terrain {
    pub fn new(n_of_peaks: i32) -> Terrain {
        let (points, landing_zone) = generate_terrain(generate_points(n_of_peaks));
        Terrain {
            points,
            landing_zone,
        }
    }

    pub fn get_points(&self) -> &Vec<Vec<i32>> {
        &self.points
    }

    pub fn get_landing_zone_points(&self) -> &Vec<Vec<i32>> {
        &self.landing_zone
    }
}

fn generate_points(n_of_peaks: i32) -> Vec<Vec<i32>> {
    let mut points: Vec<Vec<i32>> = vec![];
    for i in 0..((n_of_peaks * 2) + 1) {
        let x = (640 / (n_of_peaks * 2 - 1)) * i;
        let y = generate_y_point(80, 0);
        points.push(vec![x, y]);
    }
    points.push(vec![640, 480]);
    points.push(vec![0, 480]);
    points
}

fn generate_y_point(upper_offset: i32, lower_offset: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(upper_offset..(480 - lower_offset))
}

fn generate_terrain(mut points: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    let landing_zone_index = rng.gen_range(2..points.len() - 3);
    let previous_point = &points[landing_zone_index - 1];
    let next_point = &points[landing_zone_index];
    let (landing_zone_upper_offset, landing_zone_lower_offset) =
        get_landing_zone_height_offsets(previous_point[1], next_point[1]);
    let landing_zone_y = generate_y_point(landing_zone_upper_offset, landing_zone_lower_offset);
    let (landing_zone_start_x, landing_zone_end_x) =
        generate_landing_zone_x(previous_point[0], next_point[0]);
    let landing_zone_start = vec![landing_zone_start_x, landing_zone_y];
    let landing_zone_end = vec![landing_zone_end_x, landing_zone_y];
    points.insert(landing_zone_index.clone(), landing_zone_start.clone());
    points.insert(landing_zone_index.clone() + 1, landing_zone_end.clone());
    (points, vec![landing_zone_start, landing_zone_end])
}

fn get_landing_zone_height_offsets(previous_point_y: i32, next_point_y: i32) -> (i32, i32) {
    let max_height = 480;
    let upper_offset: i32;
    let lower_offset: i32;
    if previous_point_y > next_point_y {
        upper_offset = next_point_y;
        lower_offset = max_height - previous_point_y;
    } else {
        upper_offset = previous_point_y;
        lower_offset = 480 - next_point_y;
    }
    (upper_offset, lower_offset)
}

fn generate_landing_zone_x(previous_point_x: i32, next_point_x: i32) -> (i32, i32) {
    let landing_zone_width = 20;
    let x_offset: i32 = (next_point_x - previous_point_x - landing_zone_width) / 2;
    let start = previous_point_x + x_offset;
    (start, start + landing_zone_width)
}
