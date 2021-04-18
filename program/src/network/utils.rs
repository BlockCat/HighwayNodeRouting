use std::{collections::HashMap, str::FromStr};

use shapefile::{Polyline, dbase::FieldValue};

#[derive(Debug)]
pub enum RoadDirection {
    // Both (JTE_BEGIN <-> JTE_END) denoted with H
    BOTH,
    // Direction with flow: (JTE_BEGIN -> JTE_END) denoted with H
    WITH,
    // Direction against flow: (JTE_END -> JTE_BEGIN) denoted with T
    AGAINST,
}

impl FromStr for RoadDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(RoadDirection::WITH),
            "T" => Ok(RoadDirection::AGAINST),
            "B" => Ok(RoadDirection::BOTH),
            "O" => Ok(RoadDirection::BOTH), // O (Onbekend) means unknown but used as both directions.
            _ => Err(s.into()),
        }
    }
}

pub fn get_character<'a>(
    record: &'a HashMap<String, FieldValue>,
    key: &str,
) -> Result<&'a String, String> {
    match &record.get(key) {
        Some(FieldValue::Character(Some(value))) => Ok(value),
        _ => Err(format!("No {}: {:?}", key, record)),
    }
}

pub fn get_numeric(record: &HashMap<String, FieldValue>, key: &str) -> Result<f64, String> {
    match &record.get(key) {
        Some(FieldValue::Numeric(Some(value))) => Ok(*value),
        _ => Err(format!("No {}: {:?}", key, record)),
    }
}

pub fn calculate_distance(shape: &Polyline) -> f32 {
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