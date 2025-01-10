use iced::widget::{button, column, container, row, text, text_editor};
use iced::Length::{Fill, Shrink};
use iced::{window, Element, Font, Subscription, Task, Theme};

use crate::common_types::Index;
use crate::spreadsheet::SpreadSheet;

#[derive(Debug, Clone)]
enum Message {
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
            Message::WindowResized(_size) => Task::none(),
            Message::Edit(action) => {
                self.editor_content.perform(action);

                Task::none()
            }
            Message::CellPressed(index) => {
                if let Some(previous) = self.selected_cell {
                    if previous != index {
                        let new_content = self.editor_content.text().trim().to_string();
                        let previous_content =
                            self.spread_sheet.get_raw(&previous).unwrap_or_default();

                        match (previous_content, new_content.as_str()) {
                            (prev, new) if prev == new => (),
                            ("", "") => (),
                            ("", _added_content) => self
                                .spread_sheet
                                .add_cell_and_compute(previous, new_content),
                            (_deleted_content, "") => self.spread_sheet.remove_cell(previous),
                            (_mutated_from, _mutated_to) => {
                                self.spread_sheet.mutate_cell(previous, new_content)
                            }
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
                text_editor(&self.editor_content).on_action(Message::Edit) // .highlight("rs", highlighter::Theme::SolarizedDark)
            )
            .padding(20)
            .height(Shrink)
            .width(Fill)
            .center_x(Fill),
            container(column((0..10).map(|y| {
                row((0..10).map(|x| {
                    container(
                        button(if Some(Index { x, y }) == self.selected_cell {
                            text(self.editor_content.text())
                        } else {
                            text(self.spread_sheet.get_text(Index { x, y }))
                        })
                        .on_press(Message::CellPressed(Index { x, y }))
                        .style(|_, _| button::primary(&self.theme(), button::Status::Active)),
                    )
                    .width(Fill)
                    .height(Fill)
                    .center(Fill)
                    .style(move |_| {
                        if Some(Index { x, y }) == self.selected_cell {
                            container::bordered_box(&self.theme())
                        } else {
                            container::bordered_box(&Theme::CatppuccinFrappe)
                        }
                    })
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

pub fn start(spread_sheet: SpreadSheet) -> std::result::Result<(), iced::Error> {
    iced::application("Mini Spreadsheet", GUI::update, GUI::view)
        .theme(GUI::theme)
        .default_font(Font::MONOSPACE)
        .subscription(GUI::subscription)
        .run_with(|| GUI::new(spread_sheet))
}
