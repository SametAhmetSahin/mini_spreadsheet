use std::
    path::Path
;

use raw_spreadsheet::RawSpreadSheet;
mod raw_spreadsheet;

fn main() {
    let input = Path::new("csv").join("sum.csv");
    let raw_cells = RawSpreadSheet::new(input);
    println!("{raw_cells}");
}
