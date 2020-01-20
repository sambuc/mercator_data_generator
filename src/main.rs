#![forbid(unsafe_code)]

use std::fs::File;
use std::io::BufWriter;

use mercator_db::storage::model::v1::Shape;
use mercator_db::storage::model::*;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use serde::Serialize;
use structopt::StructOpt;

const POSITIONS_PER_SHAPE: usize = 1000;

fn get_reference_space() -> Vec<Space> {
    vec![Space {
        name: "std".to_string(),
        origin: vec![0.0, 0.0, 0.0],
        axes: vec![
            Axis {
                measurement_unit: "m".to_string(),
                graduation: Graduation {
                    set: "N".to_string(),
                    minimum: 0.0,
                    maximum: 1.0,
                    steps: 1_000_000_000,
                },
                unit_vector: vec![1.0, 0.0, 0.0],
            },
            Axis {
                measurement_unit: "m".to_string(),
                graduation: Graduation {
                    set: "N".to_string(),
                    minimum: 0.0,
                    maximum: 1.0,
                    steps: 1_000_000_000,
                },
                unit_vector: vec![0.0, 1.0, 0.0],
            },
            Axis {
                measurement_unit: "m".to_string(),
                graduation: Graduation {
                    set: "N".to_string(),
                    minimum: 0.0,
                    maximum: 1.0,
                    steps: 1_000_000_000,
                },
                unit_vector: vec![0.0, 0.0, 1.0],
            },
        ],
    }]
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

    let to = format!("{}.spaces.json", name);
    let file_out =
        File::create(&to).unwrap_or_else(|e| panic!("Unable to create file: {}: {}", to, e));
    let writer = BufWriter::new(&file_out);

    serde_json::to_writer(writer, &get_reference_space()).unwrap();
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

    SpatialObject {
        properties: Properties {
            type_name: "Feature".to_string(),
            id: format!("oid{}", die.sample(rng)),
        },
        shapes,
    }
}

fn get_space(nb_points: usize, rng: &mut ThreadRng, die: &Uniform<f64>) {
    let space_name = "std";

    let mut objects = Vec::with_capacity(nb_points);

    for _ in 0..nb_points {
        objects.push(get_point(&space_name, rng, &die));
    }

    store(format!("{}k", nb_points).as_str(), objects);
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// List of Number of features to be generated.
    datasets: Vec<usize>,
}

fn main() {
    let opt = Opt::from_args();

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0.0..1.0);

    for dataset in opt.datasets {
        get_space(dataset, &mut rng, &die);
    }
}
