#[macro_use]
extern crate serde_derive;

mod storage;

use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use std::fs::File;
use std::io::BufWriter;

use serde::Serialize;

use storage::*;

const POSITIONS_PER_SHAPE: usize = 1000;

impl SpatialObject {
    pub fn new(shapes: Vec<Shape>, id: String) -> Self {
        SpatialObject {
            shapes,
            properties: Properties {
                type_name: "Feature".to_string(),
                id,
            },
        }
    }
}

fn store<T>(name: &str, data: T)
where
    T: Serialize,
{
    let to = format!("{}.objects.json", name);
    let file_out =
        File::create(&to).unwrap_or_else(|e| panic!("Unable to create file: {}: {}", to, e));
    let writer = BufWriter::new(&file_out);

    serde_json::to_writer(writer, &data).unwrap();
}

fn get_point(space_name: &str, rng: &mut ThreadRng, die: &Uniform<f64>) -> SpatialObject {
    let mut shapes = Vec::with_capacity(POSITIONS_PER_SHAPE);

    for _ in 0..POSITIONS_PER_SHAPE {
        shapes.push(Shape {
            type_name: "Point".to_string(),
            vertices: vec![vec![die.sample(rng), die.sample(rng), die.sample(rng)]],
            reference_space: space_name.to_string(),
        });
    }

    SpatialObject::new(shapes, format!("oid{}", die.sample(rng)))
}

fn get_space(nb_points: usize, rng: &mut ThreadRng, die: &Uniform<f64>) {
    let space_name = "std";

    let mut objects = Vec::with_capacity(nb_points);

    for _ in 0..nb_points {
        objects.push(get_point(&space_name, rng, &die));
    }

    store(format!("{}k", nb_points).as_str(), objects);
}

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0.0..1.0);

    get_space(1, &mut rng, &die);
    get_space(10, &mut rng, &die);
    get_space(100, &mut rng, &die);
    get_space(1000, &mut rng, &die);
    get_space(10000, &mut rng, &die);
    //get_space(40000, &mut rng, &die);
}
