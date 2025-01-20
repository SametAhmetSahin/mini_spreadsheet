pub mod common_types;
use std::path::PathBuf;

use gui::GUI;
use spreadsheet::SpreadSheet;

mod gui;
mod renderer;
mod spreadsheet;

#[macroquad::main("MyGame")]
async fn main() {
    let spread_sheet = SpreadSheet::from_file_path(PathBuf::from("./csv/sum.csv"));
    let mut gui = GUI::new(spread_sheet).await;
    gui.start().await;
}
