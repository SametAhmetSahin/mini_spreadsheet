pub mod common_types;
use gui::GUI;
use macroquad::prelude::*;
use spreadsheet::SpreadSheet;

mod gui;
mod renderer;
mod spreadsheet;

#[macroquad::main("MyGame")]
async fn main() {
    let spread_sheet = SpreadSheet::default();
    let mut gui = GUI::new();
    gui.start().await;
}
