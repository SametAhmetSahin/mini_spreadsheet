use std::path::Path;
pub mod common_types;
use spreadsheet::SpreadSheet;

mod spreadsheet;

fn main() {
    let input = Path::new("csv").join("sum.csv");
    let mut spread_sheet = SpreadSheet::from_file_path(input);
    spread_sheet.compute_all();
    for (k, v) in spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }
}
