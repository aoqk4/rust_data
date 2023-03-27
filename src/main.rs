use polars::prelude::*;
fn main() {
    let ignore: Vec<String> = vec!["NA".to_string()];

    let df = CsvReader::from_path("/home/aoqk4/rust_data/csv/athlete_events.csv")
        .unwrap()
        .with_null_values(Some(NullValues::AllColumns(ignore)))
        .finish()
        .unwrap();

    println!("{:?}", df.head(Some(5)));

    let filt = df.column("Medal").unwrap().is_not_null();

    let res = df.filter(&filt).unwrap();

    println!("{:?}", res.head(Some(5)));
}
