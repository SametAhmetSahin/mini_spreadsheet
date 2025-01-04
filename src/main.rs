use std::path::Path;
pub mod common_types;
use spreadsheet::SpreadSheet;

mod spreadsheet;
mod renderer;

use macroquad::prelude::*;

#[macroquad::main("MiniSpreadSheet")]
async fn main() {
    let input = Path::new("csv").join("cycle.csv");
    let mut spread_sheet = SpreadSheet::from_file_path(input);
    spread_sheet.compute_all();

    for (k, v) in &spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }

    loop {
        clear_background(WHITE);

        renderer::render_spread_sheet(&spread_sheet);

        next_frame().await
    }
}
