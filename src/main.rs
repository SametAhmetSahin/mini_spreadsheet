use std::path::Path;
pub mod common_types;
use spreadsheet::SpreadSheet;

mod spreadsheet;

fn main() {
    let input = Path::new("csv").join("sum.csv");
    let spread_sheet = SpreadSheet::from_file_path(input);
    println!("{:?}",  &spread_sheet);
}
