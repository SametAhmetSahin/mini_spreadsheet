use std::path::Path;
pub mod common_types;
use common_types::Index;
use iced::{widget::text, Element, Font, Task, Theme};
use spreadsheet::SpreadSheet;

mod renderer;
mod spreadsheet;

#[derive(Debug, Clone)]
enum Message {}

struct GUI {
    spread_sheet: SpreadSheet,
}

impl GUI {
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(spread_sheet: SpreadSheet) -> (Self, Task<Message>) {
        (Self { spread_sheet }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        text!("Hello World!").into()
    }
}

fn main() -> iced::Result {
    let input = Path::new("csv").join("nested.csv");
    let mut spread_sheet = SpreadSheet::from_file_path(input);
    spread_sheet.compute_all();

    for (k, v) in &spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }

    println!("");
    spread_sheet.mutate_cell(Index { x: 0, y: 0 }, "0".to_string());

    for (k, v) in &spread_sheet.cells {
        println!("{:?} {:?}", k, v.computed_value);
    }

    iced::application("Mini Spreadsheet", GUI::update, GUI::view)
        .theme(GUI::theme)
        .default_font(Font::MONOSPACE)
        .run_with(|| GUI::new(spread_sheet))
}
