use crate::{common_types::Index, spreadsheet::SpreadSheet};
use macroquad::prelude::*;

const CELL_WIDTH: f32 = 100.0;
const CELL_HEIGHT: f32 = 50.0;
const TEXT_SIZE: f32 = 14.0;
const HEADER_BG: Color = GRAY;
const INPUT_HEIGHT: f32 = 30.0; // Height of the input bar

pub struct GUIState {
    pub user_input: String, // Store the current user input
    pub is_focused: bool,   // Track if the input bar is focused
}

impl GUIState {
    pub fn new() -> Self {
        Self {
            user_input: String::new(),
            is_focused: false,
        }
    }
}

/// Renders the input bar at the top of the screen.
fn render_input_bar(gui_state: &mut GUIState) {
    let screen_width = screen_width();

    // Draw the input bar
    draw_rectangle(0.0, 0.0, screen_width, INPUT_HEIGHT, WHITE);
    draw_rectangle_lines(0.0, 0.0, screen_width, INPUT_HEIGHT, 2.0, DARKGRAY);

    let text_x = 10.0; // Padding from the left
    let text_y = (INPUT_HEIGHT + TEXT_SIZE) / 2.0;

    // Draw the user input text
    draw_text(&gui_state.user_input, text_x, text_y, TEXT_SIZE, BLACK);

    // Handle input focus and typing
    if is_mouse_button_pressed(MouseButton::Left) {
        let (_, mouse_y) = mouse_position();
        gui_state.is_focused = mouse_y <= INPUT_HEIGHT;
    }

    // ! This consumes the chars that were pressed while unfocused, altough I am not sure if this logic should be handled here
    while let Some(key) = get_char_pressed() {
        if gui_state.is_focused {
            match key {
                '\n' => gui_state.is_focused = false, // Unfocus on Enter
                '\x08' => {
                    // Handle backspace
                    gui_state.user_input.pop();
                }
                _ => gui_state.user_input.push(key),
            }
        }
    }
}

/// Renders the column headers (A, B, C, ...).
fn render_column_headers(max_visible_x: usize) {
    for x in 0..max_visible_x {
        let col_name = (b'A' + x as u8) as char;
        let x_pos = (x + 1) as f32 * CELL_WIDTH; // Offset by one cell for row header

        // Draw header background
        draw_rectangle(x_pos, INPUT_HEIGHT, CELL_WIDTH, CELL_HEIGHT, HEADER_BG);

        // Center text within the header cell
        let text_dimensions = measure_text(&col_name.to_string(), None, TEXT_SIZE as u16, 1.0);
        let text_x = x_pos + (CELL_WIDTH - text_dimensions.width) / 2.0;
        let text_y = INPUT_HEIGHT + (CELL_HEIGHT + text_dimensions.height) / 2.0;

        draw_text(&col_name.to_string(), text_x, text_y, TEXT_SIZE, WHITE);
    }
}

/// Renders the row headers (1, 2, 3, ...).
fn render_row_headers(max_visible_y: usize) {
    for y in 0..max_visible_y {
        let row_name = (y + 1).to_string();
        let y_pos = INPUT_HEIGHT + (y + 1) as f32 * CELL_HEIGHT; // Offset by input and column headers

        // Draw header background
        draw_rectangle(0.0, y_pos, CELL_WIDTH, CELL_HEIGHT, HEADER_BG);

        // Center text within the header cell
        let text_dimensions = measure_text(&row_name, None, TEXT_SIZE as u16, 1.0);
        let text_x = (CELL_WIDTH - text_dimensions.width) / 2.0;
        let text_y = y_pos + (CELL_HEIGHT + text_dimensions.height) / 2.0;

        draw_text(&row_name, text_x, text_y, TEXT_SIZE, WHITE);
    }
}

/// Renders the spreadsheet cells.
fn render_cells(spreadsheet: &SpreadSheet, max_visible_x: usize, max_visible_y: usize) {
    for y in 0..max_visible_y {
        for x in 0..max_visible_x {
            let index = Index { x, y };
            let x_pos = (x + 1) as f32 * CELL_WIDTH; // Offset by one cell for row header
            let y_pos = INPUT_HEIGHT + (y + 1) as f32 * CELL_HEIGHT; // Offset by input and headers

            // Draw the cell rectangle
            draw_rectangle(x_pos, y_pos, CELL_WIDTH, CELL_HEIGHT, LIGHTGRAY);
            draw_rectangle_lines(x_pos, y_pos, CELL_WIDTH, CELL_HEIGHT, 2.0, DARKGRAY);

            // Only render text for computed cells
            if let Some(computed) = spreadsheet.get_computed(index) {
                let content = match computed {
                    Ok(val) => val.to_string(),
                    Err(err) => err.to_string(),
                };

                // Center text within the cell
                let text_dimensions = measure_text(&content, None, TEXT_SIZE as u16, 1.0);
                let text_x = x_pos + (CELL_WIDTH - text_dimensions.width) / 2.0;
                let text_y = y_pos + (CELL_HEIGHT + text_dimensions.height) / 2.0;

                draw_text(&content, text_x, text_y, TEXT_SIZE, BLACK);
            }
        }
    }
}

/// Main rendering function.
pub fn render_spread_sheet(spreadsheet: &SpreadSheet, gui_state: &mut GUIState) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Determine the range of indices visible on the screen
    let max_visible_x = (screen_width / CELL_WIDTH).ceil() as usize;
    let max_visible_y = ((screen_height - INPUT_HEIGHT) / CELL_HEIGHT).ceil() as usize;

    // Render individual components
    render_input_bar(gui_state);
    render_column_headers(max_visible_x);
    render_row_headers(max_visible_y);
    render_cells(spreadsheet, max_visible_x, max_visible_y);
}
