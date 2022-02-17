#[macro_use] extern crate prettytable;
use std::env;
use serde::Serialize;
use serde::Deserialize;
use prettytable::{Table, Row, Cell};
use std::fs;

// Pastry struct
#[derive(Serialize, Deserialize, Debug)]
struct Pastry {
    Id: String,
    Type: String,
    Name: String,
    Batter: String,
    Topping: String
}

// TODO: Exception handling and unit tests
fn main() {

    // Usage: [path to your program] [input file path] [output file path]
    let args: Vec<String> = env::args().collect();

    let source_json_file = &args[1];
    let dest_table_file = &args[2];

    // Read the content of JSON file into a string
    let json_string = fs::read_to_string(source_json_file).expect("Unable to read JSON file");

    // Deserialize JSON into a collection of pastries
    let mut pastries: Vec<Pastry> = serde_json::from_str(&json_string).unwrap();

    // Sort pastries by Id in ascending order
    pastries.sort_by(|a, b| a.Id.cmp(&b.Id));

    let mut table = Table::new();

    // Table header
    table.add_row(row!["Id", "Type", "Name", "Batter", "Topping"]);
    for pastry in pastries {
        // Add a row per time
        table.add_row(row![pastry.Id, pastry.Type, pastry.Name, pastry.Batter, pastry.Topping]);
    }
    fs::write(dest_table_file,  table.to_string()).expect("Unable to write file");
}