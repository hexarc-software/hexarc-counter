use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new_random() -> Self {
        Self {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
        }
    }
}