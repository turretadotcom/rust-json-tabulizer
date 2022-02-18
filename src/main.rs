#[macro_use] extern crate prettytable;

use std::cmp::Ordering;
use std::env;
use serde::Serialize;
use serde::Deserialize;
use prettytable::{Table, Row, Cell};
use std::fs;
use std::path::Path;

static SORT_COMMAND_KEYWORD: &str = "sortby";
static DEFAULT_SORT_KEY: &str = "Id";
static AVAILABLE_SORT_KEYS:[&str; 5] = ["Id", "Type", "Name", "Batter", "Topping"];

/// Pastry struct
#[derive(Serialize, Deserialize, Debug)]
struct Pastry {
    Id: String,
    Type: String,
    Name: String,
    Batter: String,
    Topping: String
}

/// Retrieve sort key if sort command is used in the destination file
fn get_sort_key_from_filename(dest_filename: &String) -> &str {
    let path:&Path = Path::new(dest_filename);
    let filename:&str = path.file_name().unwrap().to_str().unwrap();
    let mut sort_key = DEFAULT_SORT_KEY;

    // Loop through available sort keys and return match
    if filename.contains(SORT_COMMAND_KEYWORD) {
        for available_for_key in AVAILABLE_SORT_KEYS {
            if filename.to_lowercase().contains(&available_for_key.to_lowercase()) {
                return available_for_key;
            }
        }
    }
    return sort_key;
}

/// Sort pastries by sort key
fn sort_pastries(pastries: &mut Vec<Pastry>, sort_key: &str) {

    let sort_closure_operation:fn(first: &Pastry, second: &Pastry)-> Ordering = match sort_key {
        "Id" => |a, b| a.Id.cmp(&b.Id),
        "Type" => |a, b| a.Type.cmp(&b.Type),
        "Name" => |a, b| a.Name.cmp(&b.Name),
        "Batter" => |a, b| a.Batter.cmp(&b.Batter),
        "Topping" => |a, b| a.Topping.cmp(&b.Topping),
        _ => |a, b| a.Id.cmp(&b.Id),
    };

    // Sort pastries by chosen closure operation
    pastries.sort_by(sort_closure_operation);
}

/// TODO: Exception handling and unit tests
fn main() {

    // Usage: [path to your program] [input file path] [output file path]
    let args: Vec<String> = env::args().collect();

    let source_json_file:&String = &args[1];
    let dest_table_file:&String = &args[2];

    let sort_key = get_sort_key_from_filename(dest_table_file);

    // Read the content of JSON file into a string
    let json_string = fs::read_to_string(source_json_file).expect("Unable to read JSON file");

    // Deserialize JSON into a collection of pastries
    let mut pastries: Vec<Pastry> = serde_json::from_str(&json_string).unwrap();

    sort_pastries(&mut pastries, &sort_key);

    let mut table = Table::new();

    // Table header
    table.add_row(row!["Id", "Type", "Name", "Batter", "Topping"]);
    for pastry in pastries {
        // Add a row per time
        table.add_row(row![pastry.Id, pastry.Type, pastry.Name, pastry.Batter, pastry.Topping]);
    }
    fs::write(dest_table_file,  table.to_string()).expect("Unable to write file");
}