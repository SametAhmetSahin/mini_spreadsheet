use std::{
    fmt::Display,
    fs::File,
    io::Read,
    path::PathBuf,
};

pub struct RawCell(pub String);

/// Represents a spread sheet where the inputs have not been processed
pub struct RawSpreadSheet {
    pub rows: Vec<Vec<RawCell>>,
    pub height: usize,
    pub width: usize,
}

impl RawSpreadSheet {
    pub fn new(input_path: PathBuf) -> Self {
        let mut buffer = String::new();
        let mut f = File::open(input_path).expect("Cannot open file");
        f.read_to_string(&mut buffer)
            .expect("Cannot read file to string");

        let rows: Vec<Vec<RawCell>> = buffer
            .lines()
            .map(|x| x.split('|').map(|s| RawCell(s.to_string())).collect())
            .collect();

        let width = rows
            .iter()
            .map(Vec::len)
            .max()
            .expect("Expected at least one column");
        let height = rows.len();

        RawSpreadSheet {
            rows,
            height,
            width,
        }
    }
}

impl Display for RawSpreadSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.rows.get(y).and_then(|row| row.get(x)) {
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
