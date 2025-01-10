use std::path::Path;
pub mod common_types;
use spreadsheet::SpreadSheet;

mod gui;
mod renderer;
mod spreadsheet;

fn main() -> iced::Result {
    let spread_sheet = SpreadSheet::default();
    // let input = Path::new("csv").join("nested.csv");
    // let mut spread_sheet = SpreadSheet::from_file_path(input);
    // spread_sheet.compute_all();

    gui::start(spread_sheet)
}
