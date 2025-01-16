use camera::mouse;
use macroquad::prelude::*;

pub struct GUI {
    selected_cell: Option<(u16, u16)>,
    last_click: Option<(f32, f32)>,
}

impl GUI {
    pub fn new() -> Self {
        Self {
            last_click: None,
            selected_cell: None,
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

            // draw
            clear_background(RED);

            self.draw_horizontal_layout((0.0, 0.0), (screen_width(), screen_height()));

            next_frame().await
        }
    }

    fn draw_editor(&mut self, start: (f32, f32), end: (f32, f32)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        draw_rectangle(start_x, start_y, end_x - start_x, end_y - start_y, GREEN);
        draw_rectangle_lines(
            start_x,
            start_y,
            end_x - start_x,
            end_y - start_y,
            2.0,
            DARKGRAY,
        );
    }

    fn draw_cells(&mut self, start: (f32, f32), end: (f32, f32)) {
        let (start_x, start_y) = start;
        let (end_x, end_y) = end;
        let num_cells_vertical = 10;
        let num_cells_horizontal = 10; 

        let cell_height = (end_y - start_y) / num_cells_vertical as f32;
        let cell_width = (end_x - start_x) / num_cells_horizontal as f32;

        // Draw background
        draw_rectangle(start_x, start_y, end_x - start_x, end_y - start_y, WHITE);

        // Draw all cells in the grid
        for row in 0..num_cells_vertical {
            for col in 0..num_cells_horizontal {
                let cell_start_x = start_x + col as f32 * cell_width;
                let cell_start_y = start_y + row as f32 * cell_height;

                Self::draw_cell((cell_start_x, cell_start_y), (cell_width, cell_height));
            }
        }
    }

    fn draw_cell(start: (f32, f32), dimensions: (f32, f32)) {
        let (start_x, start_y) = start;
        let (width, height) = dimensions;

        // Draw cell outline with the basic border color
        draw_rectangle_lines(start_x, start_y, width, height, 1.0, BLACK);
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
}
