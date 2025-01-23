pub mod common_types;

use gui::GUI;
use spreadsheet::SpreadSheet;

mod gui;
mod renderer;
mod spreadsheet;

#[macroquad::main("MyGame")]
async fn main() {
    let spread_sheet = SpreadSheet::default();
    let mut gui = GUI::new(spread_sheet).await;
    gui.start().await;
}
