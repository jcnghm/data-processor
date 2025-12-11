use csv_to_json::insert_value;
use serde_json::{Map, Value};
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // Get command line args
    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);
    let mut reader = csv::Reader::from_path(args[1].clone())?;
    let headers = reader.headers()?.clone();
    let mut records = Vec::new();

    // print headers
    println!("Headers: {:?}", headers);
    println!("--------------------------------");

    for result in reader.records() {
        let record = result?;
        println!("Record: {:?}", record);

        let mut root_obj = Map::new();

        for (header, field) in headers.iter().zip(record.iter()) {
            insert_value(&mut root_obj, &header, field);
        }

        records.push(Value::Object(root_obj));
    }

    let json = Value::Array(records);
    let json_string = serde_json::to_string_pretty(&json)?;
    println!("JSON: {}", json_string);

    Ok(())
}
