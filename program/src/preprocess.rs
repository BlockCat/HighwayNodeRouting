use crate::network::{Network, Writeable};
use shapefile::{reader::ShapeRecordIterator, Polyline};
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub fn preprocess<S>() -> Result<S, Box<dyn Error>>
where
    S: Writeable + Network,
{
    let input = "./data/Wegvakken/Wegvakken.shp";
    let output = "./data/network3.axe";
    preprocess_network(input, output)
}

fn preprocess_network<P, S>(input: P, output: P) -> Result<S, Box<dyn Error>>
where
    P: AsRef<Path>,
    S: Writeable + Network,
{
    if File::open(output.as_ref()).is_ok() {
        println!("Output exists, already preprocessed");
        return S::read(output.as_ref());
    }

    let shapes = read_shapes(input)?;

    println!("No output exists, creating preprocessed");
    let writeable: S = shapes.into();
    writeable.write(output)?;
    Ok(writeable)
}

fn read_shapes<P: AsRef<Path>>(
    path: P,
) -> Result<ShapeRecordIterator<BufReader<File>, Polyline>, shapefile::Error> {
    shapefile::Reader::from_path(path)
        .and_then(|reader| reader.iter_shapes_and_records_as::<Polyline>())
}
