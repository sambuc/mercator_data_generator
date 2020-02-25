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

fn get_axis(
    unit_vector: Vec<f64>,
    minimum: f64,
    maximum: f64,
    steps: u64,
    measurement_unit: &str,
    set: &str,
) -> Axis {
    Axis {
        measurement_unit: measurement_unit.to_string(),
        graduation: Graduation {
            set: set.to_string(),
            minimum,
            maximum,
            steps,
        },
        unit_vector,
    }
}

fn get_reference_space() -> Vec<Space> {
    vec![Space {
        name: "std".to_string(),
        origin: vec![0.0, 0.0, 0.0],
        axes: vec![
            get_axis(vec![1.0, 0.0, 0.0], 0.0, 1.0, 1_000_000_000, "m", "R"),
            get_axis(vec![0.0, 1.0, 0.0], 0.0, 1.0, 1_000_000_000, "m", "R"),
            get_axis(vec![0.0, 0.0, 1.0], 0.0, 1.0, 1_000_000_000, "m", "R"),
        ],
    }]
}

fn store<T>(basename: &str, data: T)
where
    T: Serialize,
{
    let to = format!("{}.json", basename);
    let file_out =
        File::create(&to).unwrap_or_else(|e| panic!("Unable to create file '{}': {}", to, e));
    let writer = BufWriter::new(&file_out);

    serde_json::to_writer(writer, &data)
        .unwrap_or_else(|e| panic!("Unable to serialize to '{}': {}", to, e));
}

fn generate_points(
    factor: usize,
    space_name: &str,
    rng: &mut ThreadRng,
    die: &Uniform<f64>,
) -> Vec<SpatialObject> {
    let mut shapes = Vec::with_capacity(POSITIONS_PER_SHAPE);
    let mut v = Vec::with_capacity(factor);

    for _ in 0..POSITIONS_PER_SHAPE {
        shapes.push(Shape {
            type_name: "Point".to_string(),
            vertices: vec![vec![die.sample(rng), die.sample(rng), die.sample(rng)]],
            reference_space: space_name.to_string(),
        });
    }

    for _ in 0..(factor - 1) {
        v.push(SpatialObject {
            properties: Properties {
                type_name: "Feature".to_string(),
                id: format!("oid{}", die.sample(rng)),
            },
            shapes: shapes.clone(),
        });
    }

    // Last overlaping point can own the vector of position, saves a clone which
    // would be simply discarded right away.
    v.push(SpatialObject {
        properties: Properties {
            type_name: "Feature".to_string(),
            id: format!("oid{}", die.sample(rng)),
        },
        shapes,
    });

    v
}

fn generate_data(nb_points: usize, factor: usize, rng: &mut ThreadRng, die: &Uniform<f64>) {
    let space_name = "std";

    let mut objects = Vec::with_capacity(nb_points);

    // First Serialize the space definition
    store(
        format!("{}k.spaces", nb_points).as_str(),
        get_reference_space(),
    );

    // Now store the generated dataset
    for _ in 0..nb_points {
        objects.append(&mut generate_points(factor, &space_name, rng, &die));
    }

    store(format!("{}k.objects", nb_points).as_str(), objects);
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Number of ids per positions generated
    #[structopt(long, short)]
    factor: Option<usize>,

    /// List of Number of features to be generated.
    datasets: Vec<usize>,
}

fn main() {
    let opt = Opt::from_args();

    let factor = match opt.factor {
        None => 1,
        Some(val) => val,
    };

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0.0..1.0);

    for dataset in opt.datasets {
        generate_data(dataset, factor, &mut rng, &die);
    }
}
