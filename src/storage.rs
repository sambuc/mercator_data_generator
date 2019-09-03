#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Space {
    pub name: String,
    pub origin: Vec<f64>,
    pub axes: Vec<Axis>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Axis {
    pub measurement_unit: String,
    pub graduation: Graduation,
    pub unit_vector: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Graduation {
    pub set: String,
    pub minimum: f64,
    pub maximum: f64,
    pub steps: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpatialObject {
    pub properties: Properties,
    pub shapes: Vec<Shape>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shape {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(rename = "space")]
    pub reference_space: String,
    pub vertices: Vec<Point>,
}

type Point = Vec<f64>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Properties {
    #[serde(rename = "type")]
    pub type_name: String,
    pub id: String,
}
