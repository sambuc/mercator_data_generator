#[macro_use]
extern crate serde_derive;

mod storage;

use rand::distributions::{Distribution, Uniform};

use rand::prelude::ThreadRng;
use storage::*;

const POSITIONS_PER_SHAPE: usize = 1000;

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

    storage::store(format!("{}k", nb_points).as_str(), objects);
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
