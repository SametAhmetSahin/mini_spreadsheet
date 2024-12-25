use std::{cell, path::Path};

use parser::CellParser;
use raw_spreadsheet::RawSpreadSheet;
mod raw_spreadsheet;
mod parser;

fn main() {
    let input = Path::new("csv").join("sum.csv");
    let raw_cells = RawSpreadSheet::new(input);
    println!("{}", &raw_cells);
    let parsed_cells = CellParser::parse_raw(raw_cells);
    println!("{:?}", parsed_cells);
}
