use std::path::Path;
pub mod common_types;
use common_types::Index;
use iced::widget::text_editor::Edit;
use iced::widget::{button, column, container, row, text, text_editor};
use iced::Length::{Fill, Shrink};
use iced::{window, Border, Color, Element, Font, Subscription, Task, Theme};
use spreadsheet::SpreadSheet;

mod renderer;
mod spreadsheet;

#[derive(Debug, Clone)]
enum Message {
    AddRaw { idx: Index, raw: String },
    WindowResized(iced::Size),
    Edit(text_editor::Action),
    CellPressed(Index),
}

struct GUI {
    spread_sheet: SpreadSheet,
    editor_content: text_editor::Content,
    selected_cell: Option<Index>,
}

impl GUI {
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(spread_sheet: SpreadSheet) -> (Self, Task<Message>) {
        (
            Self {
                spread_sheet,
                editor_content: text_editor::Content::default(),
                selected_cell: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddRaw { idx, raw } => {
                self.spread_sheet.add_cell_and_compute(idx, raw);
                Task::none()
            }

            Message::WindowResized(_size) => Task::none(),
            Message::Edit(action) => {
                self.editor_content.perform(action);

                Task::none()
            }
            Message::CellPressed(index) => {
                if let Some(previous) = self.selected_cell {
                    if previous != index {
                        let new_content = self.editor_content.text().trim().to_string();
                        let previous_content = self.spread_sheet.get_raw(&previous).unwrap_or_default();
                        println!("The editor has the content: {new_content:?}, the cell had content {previous_content:?}");
                        if new_content != previous_content {
                            println!("Mutated!: idx: {previous:?}  to: {new_content}");
                            self.spread_sheet.mutate_cell(previous, new_content);
                        }
                    }
                } 
                self.selected_cell = Some(index);

                if let Some(raw) = self.spread_sheet.get_raw(&index) {
                    self.editor_content = text_editor::Content::with_text(raw);
                } else {
                    self.editor_content = text_editor::Content::default();
                }

                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_id, size)| Message::WindowResized(size))
    }

    fn view(&self) -> Element<Message> {
        container(column![
                container(
                    text_editor(&self.editor_content)
                        .placeholder("Type something here...")
                        .on_action(Message::Edit)
                )
                .padding(20)
                .height(Shrink)
                .width(Fill)
                .center_x(Fill)
                .style(|_| container::Style::default()
                    .background(iced::Color::new(1.0, 0.0, 0.0, 1.0))),
                container(column((0..10).map(|y| {
                    row((0..10).map(|x| {
                        container(
                            button(text(self.spread_sheet.get_text(Index { x, y })))
                                .on_press(Message::CellPressed(Index { x, y })),
                        )
                        .width(Fill)
                        .height(Fill)
                        .center(Fill)
                        .style(|_| container::bordered_box(&self.theme()))
                        .into()
                    }))
                    .width(Fill)
                    .height(Fill)
                    .into()
                })))
                .height(Fill)
                .width(Fill)
                .center(Fill)
                .padding(10u16)
            ])
        .into()
    }
}

fn main() -> iced::Result {
    let input = Path::new("csv").join("nested.csv");
    let mut spread_sheet = SpreadSheet::from_file_path(input);
    spread_sheet.compute_all();

    iced::application("Mini Spreadsheet", GUI::update, GUI::view)
        .theme(GUI::theme)
        .default_font(Font::MONOSPACE)
        .subscription(GUI::subscription)
        .run_with(|| GUI::new(spread_sheet))
}
