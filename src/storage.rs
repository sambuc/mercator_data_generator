use std::fs::File;
use std::io::BufWriter;

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Space {
    pub name: String,
    pub system: CoordinateSystem,
}

#[derive(Clone, Debug, Serialize)]
pub struct CoordinateSystem {
    pub origin: Vec<f64>,
    pub axes: Vec<Axis>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Axis {
    pub measurement_unit: String,
    pub graduation: Graduation,
    pub unit_vector: Vec<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Graduation {
    pub set: String,
    pub minimum: f64,
    pub maximum: f64,
    pub steps: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SpatialObject {
    pub properties: Properties,
    pub shapes: Vec<Shape>,
}

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

#[derive(Clone, Debug, Serialize)]
pub struct Shape {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(rename = "space")]
    pub reference_space: String,
    pub vertices: Vec<Point>,
}

type Point = Vec<f64>;

#[derive(Clone, Debug, Serialize)]
pub struct Properties {
    #[serde(rename = "type")]
    pub type_name: String,
    pub id: String,
}

mod json {
    use super::*;

    pub fn store<T>(data: T, to: &str)
    where
        T: Serialize,
    {
        let file_out =
            File::create(to).unwrap_or_else(|e| panic!("Unable to create file: {}: {}", to, e));

        // We create a buffered writer from the file we get
        let writer = BufWriter::new(&file_out);

        serde_json::to_writer(writer, &data).unwrap();
    }
}

pub fn store<S, T>(name: S, data: T)
where
    S: Into<String>,
    T: Serialize,
{
    let name = name.into();
    /*
        // Convert Reference Space definitions
        let fn_out = format!("{}.spaces.json", name);

        json::store::<Vec<json::Space>>(data, &fn_out);
    */
    // Convert Spatial Objects
    let fn_out = format!("{}.objects.json", name);

    json::store(data, &fn_out);
}
