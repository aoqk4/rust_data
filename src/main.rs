use polars::prelude::*;
fn main() {
    let ignore: Vec<String> = vec!["NA".to_string()];

    let athletes = CsvReader::from_path("/home/aoqk4/rust_data/csv/athlete_events.csv")
        .unwrap()
        .with_null_values(Some(NullValues::AllColumns(ignore)))
        .finish()
        .unwrap();

    let regions = CsvReader::from_path("/home/aoqk4/rust_data/csv/noc_regions.csv")
        .unwrap()
        .finish()
        .unwrap();

    // TODO---------------------- left_join 해보자.. -----------------------------------------------------------------------

    let mut athletes_merge = athletes.left_join(&regions, ["NOC"], ["NOC"]).unwrap();

    let mut file = std::fs::File::create("path.csv").unwrap();
    CsvWriter::new(&mut file)
        .finish(&mut athletes_merge)
        .unwrap();
}
