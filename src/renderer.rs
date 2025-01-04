use macroquad::prelude::*;

use crate::{common_types::Index, spreadsheet::SpreadSheet};
const CELL_WIDTH: f32 = 100.0;
const CELL_HEIGHT: f32 = 50.0;
const TEXT_SIZE : f32 = 14.0;

pub fn render_spread_sheet(spreadsheet: &SpreadSheet) {

    let screen_width = screen_width();
    let screen_height = screen_height();

    // Determine the range of indices visible on the screen
    let max_visible_x = (screen_width / CELL_WIDTH).ceil() as usize;
    let max_visible_y = (screen_height / CELL_HEIGHT).ceil() as usize;

    // Iterate through visible cells
    for y in 0..max_visible_y {
        for x in 0..max_visible_x {
            let index = Index { x, y };
            let x_pos = x as f32 * CELL_WIDTH;
            let y_pos = y as f32 * CELL_HEIGHT;

            // Draw the cell rectangle
            draw_rectangle(x_pos, y_pos, CELL_WIDTH, CELL_HEIGHT, LIGHTGRAY);
            draw_rectangle_lines(x_pos, y_pos, CELL_WIDTH, CELL_HEIGHT, 2.0, DARKGRAY);

            // Only render text for computed cells
            if let Some(computed) = spreadsheet.get_computed(index) {
                    let content = match computed{
                        Ok(val) => val.to_string(),
                        Err(err) => err.to_string(),
                    };

                    // Center text within the cell
                    
                    let text_dimensions = measure_text(&content, None, TEXT_SIZE as u16, 1.0);
                    let text_x = x_pos + (CELL_WIDTH - text_dimensions.width) / 2.0;
                    let text_y = y_pos + (CELL_HEIGHT + text_dimensions.height) / 2.0;

                    // Draw the text
                    draw_text(&content, text_x, text_y, TEXT_SIZE, BLACK);
                
            }
        }
    }
}
