use crate::network::{AoSNetwork, BuildEdge, BuildNode, RoadDirection};
use shapefile::{dbase::FieldValue, reader::ShapeRecordIterator, Polyline};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

const NODE_START: &'static str = "JTE_ID_BEG";
const NODE_END: &'static str = "JTE_ID_END";
const DIRECTION: &'static str = "RIJRICHTNG";

pub fn preprocess() -> Result<AoSNetwork, Box<dyn Error>> {
    let input = "./data/Wegvakken/Wegvakken.shp";
    let output = "./data/network.axe";
    preprocess_base_network(input, output)
}

fn preprocess_base_network<P: AsRef<Path>>(
    input: P,
    output: P,
) -> Result<AoSNetwork, Box<dyn Error>> {
    if File::open(output.as_ref()).is_ok() {
        println!("Output exists, already preprocessed");
        return AoSNetwork::read(output.as_ref());
    }

    println!("No output exists, creating preprocessed");

    let shapes = read_shapes(input)?;

    let mut network = AoSNetwork::new();

    for entry in shapes {
        let (shape, record) = entry?;
        let direction: RoadDirection = get_character(&record, DIRECTION).unwrap().parse().unwrap();
        let node_start = get_numeric(&record, NODE_START).unwrap() as usize;
        let node_end = get_numeric(&record, NODE_END).unwrap() as usize;

        let node_start = network.add_node(BuildNode {
            junction_id: node_start,
        });
        let node_end = network.add_node(BuildNode {
            junction_id: node_end,
        });

        network.add_edge(BuildEdge {
            source_node: node_start,
            target_node: node_end,
            direction,
            distance: calculate_distance(&shape),
        });
    }

    network.write(output)?;

    Ok(network)
}

fn calculate_distance(shape: &Polyline) -> f32 {
    let mut sum = 0f32;

    for s in shape.parts() {
        let mut iter = s.iter();
        let mut previous_point = iter.next().unwrap();
        for point in iter {
            let distance = ((previous_point.x - point.x).powi(2)
                + (previous_point.y - point.y).powi(2))
            .sqrt() as f32;

            sum += distance;
            previous_point = point;
        }
    }

    sum
}

fn get_character<'a>(
    record: &'a HashMap<String, FieldValue>,
    key: &str,
) -> Result<&'a String, String> {
    match &record.get(key) {
        Some(FieldValue::Character(Some(value))) => Ok(value),
        _ => Err(format!("No {}: {:?}", key, record)),
    }
}

fn get_numeric(record: &HashMap<String, FieldValue>, key: &str) -> Result<f64, String> {
    match &record.get(key) {
        Some(FieldValue::Numeric(Some(value))) => Ok(*value),
        _ => Err(format!("No {}: {:?}", key, record)),
    }
}

fn read_shapes<P: AsRef<Path>>(
    path: P,
) -> Result<ShapeRecordIterator<BufReader<File>, Polyline>, shapefile::Error> {
    shapefile::Reader::from_path(path)
        .and_then(|reader| reader.iter_shapes_and_records_as::<Polyline>())
}
