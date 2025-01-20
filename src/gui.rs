use camera::mouse;
use macroquad::prelude::*;

use crate::common_types::Index;

pub struct GUI {
    selected_cell: Option<Index>,
    editor_content: String,
    is_editor_focused: bool,
    last_click: Option<(f32, f32)>,
    font: Font,
}

impl GUI {
    pub async fn new() -> Self {
        let font = load_ttf_font("fonts/jetbrains-mono-font/JetbrainsMonoRegular-RpvmM.ttf")
            .await
            .unwrap();
        Self {
            last_click: None,
            selected_cell: None,
            is_editor_focused: false,
            font,
            editor_content: String::new(),
        }
    }

    pub async fn start(&mut self) {
        request_new_screen_size(1200.0, 900.0);

        loop {
            // events
            if is_mouse_button_pressed(MouseButton::Left) {
                let pos = mouse_position();
                self.last_click = Some(pos);
            }
            while let Some(c) = get_char_pressed() {
                if self.is_editor_focused {
                    if c == '\u{8}' {
                        // '\u{8}' represents the Backspace key
                        self.editor_content.pop();
                    } else if c == '\r' {
                        self.commit_editor();
                        self.is_editor_focused = false;
                    } else {
                        self.editor_content.push(c);
                    }
                }
            }

            // draw
            clear_background(RED);
            self.draw_horizontal_layout((0.0, 0.0), (screen_width(), screen_height()));

            // Clear this frame
            self.last_click = None; // If this was somehow not consumed it should not persist in the next frame

            next_frame().await
        }
    }

    fn draw_editor(&mut self, start: (f32, f32), end: (f32, f32)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        let padding = 10.0;

        if let Some(_) = self
            .last_click
            .take_if(|click| is_point_in_rect(*click, start, end))
        {
            self.is_editor_focused = true
        }

        draw_rectangle(start_x, start_y, end_x - start_x, end_y - start_y, GREEN);
        draw_rectangle_lines(
            start_x,
            start_y,
            end_x - start_x,
            end_y - start_y,
            2.0,
            DARKGRAY,
        );

        let text_start_x = start_x + padding;
        let text_start_y = (start_y + end_y) / 2.0;
        let font_size = 16;
        draw_text_ex(
            &self.editor_content,
            text_start_x,
            text_start_y,
            TextParams {
                font: Some(&self.font),
                font_size: font_size,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: BLACK,
            },
        );
    }

    fn draw_cells(&mut self, start: (f32, f32), end: (f32, f32)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        let num_cells_vertical = 10;
        let num_cells_horizontal = 10;

        let cell_height = (end_y - start_y) / num_cells_vertical as f32;
        let cell_width = (end_x - start_x) / num_cells_horizontal as f32;

        // Handle if mouse clicked
        if let Some(clicked) = self
            .last_click
            .take_if(|click| is_point_in_rect(*click, start, end))
        {
            let (x, y) = clicked;
            let (col, row) = (
                ((x - start_x) / cell_width) as i32,
                ((y - start_y) / cell_height) as i32,
            );
            self.selected_cell = Some(Index {
                x: col.try_into().expect("Got negative idx from click"),
                y: row.try_into().expect("Got negative idx from click"),
            });
        }

        // Draw background
        draw_rectangle(start_x, start_y, end_x - start_x, end_y - start_y, WHITE);

        // Draw all cells in the grid
        for row in 0..num_cells_vertical {
            for col in 0..num_cells_horizontal {
                let cell_start_x = start_x + col as f32 * cell_width;
                let cell_start_y = start_y + row as f32 * cell_height;

                self.draw_cell(
                    Index { x: col, y: row },
                    (cell_start_x, cell_start_y),
                    (cell_width, cell_height),
                );
            }
        }
    }

    fn draw_cell(&self, index: Index, start: (f32, f32), dimensions: (f32, f32)) {
        let (start_x, start_y) = start;
        let (width, height) = dimensions;

        if Some(index) == self.selected_cell {
            // Draw cell outline with the basic border color
            draw_rectangle_lines(start_x, start_y, width, height, 3.0, ORANGE);
        } else {
            // Draw cell outline with the basic border color
            draw_rectangle_lines(start_x, start_y, width, height, 1.0, BLACK);
        }
    }

    fn draw_horizontal_layout(&mut self, start: (f32, f32), end: (f32, f32)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;

        // Define the components and their relative sizes
        let draws = vec![
            (GUI::draw_editor as fn(&mut GUI, (f32, f32), (f32, f32)), 2),
            (GUI::draw_cells as fn(&mut GUI, (f32, f32), (f32, f32)), 10),
        ];

        // Distribute the total area according to the second element of the tuple
        let total_weight: u16 = draws.iter().map(|(_, weight)| weight).sum();

        // Draw each component
        let mut current_y = start_y;
        for (draw_fn, weight) in draws {
            let height = (end_y - start_y) * (weight as f32 / total_weight as f32);
            draw_fn(self, (start_x, current_y), (end_x, current_y + height));
            current_y += height;
        }
    }

    fn commit_editor(&mut self) {
        todo!()
    }
}

fn is_point_in_rect<T: std::cmp::PartialOrd>(
    point: (T, T),
    rect_start: (T, T),
    rect_end: (T, T),
) -> bool {
    rect_start.0 <= point.0
        && point.0 <= rect_end.0
        && rect_start.1 <= point.1
        && point.1 <= rect_end.1
}
