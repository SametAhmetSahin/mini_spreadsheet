use std::{cmp::max, collections::HashMap, fmt::Display, fs::File, io::Read, path::PathBuf};

pub struct RawCell(pub String);

#[derive(PartialEq, Hash, Eq, Debug)]
pub struct Index {
    x: usize,
    y: usize,
}

/// Represents a spread sheet where the inputs have not been processed
pub struct RawSpreadSheet {
    pub cells: HashMap<Index, RawCell>,
    pub height: usize,
    pub width: usize,
}

impl RawSpreadSheet {
    pub fn new(input_path: PathBuf) -> Self {
        let mut buffer = String::new();
        let mut f = File::open(input_path).expect("Cannot open file");
        f.read_to_string(&mut buffer)
            .expect("Cannot read file to string");

        let mut cells = HashMap::new();

        let (mut max_x, mut max_y) = (0, 0);

        for (y, line) in buffer.lines().enumerate() {
            for (x, cell) in line.split('|').enumerate() {
                let cell = cell.trim().to_string();
                if cell.is_empty(){
                    continue;
                }
                cells.insert(Index { x, y }, RawCell(cell));
                max_x = max(x, max_x);
                max_y = max(y, max_y);
            }
        }

        RawSpreadSheet {
            cells,
            height: max_y,
            width: max_x,
        }
    }
}

impl Display for RawSpreadSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.cells.get(&Index { x, y }) {
                    formatted.push_str(&cell.0);
                }
                if x < self.width - 1 {
                    formatted.push('|');
                }
            }
            formatted.push('\n');
        }
        write!(f, "{formatted}")
    }
}
