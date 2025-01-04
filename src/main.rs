use std::path::Path;
pub mod common_types;
use common_types::Index;
use spreadsheet::SpreadSheet;

mod renderer;
mod spreadsheet;

fn main() {
    let input = Path::new("csv").join("nested.csv");
    let mut spread_sheet = SpreadSheet::from_file_path(input);
    spread_sheet.compute_all();
    

    for (k, v) in &spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }

    println!("");
    spread_sheet.mutate_cell(Index{x: 0, y: 0}, "0".to_string());

    for (k, v) in &spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }
}
