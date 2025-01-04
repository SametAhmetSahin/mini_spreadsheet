use std::path::Path;
pub mod common_types;
use renderer::GUIState;
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

    let mut gui_state = GUIState::new();

    loop {
        clear_background(WHITE);

        renderer::render_spread_sheet(&spread_sheet,  &mut gui_state);

        next_frame().await
    }
}
