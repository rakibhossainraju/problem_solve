type SheetData = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct Spreadsheet {
    data: SheetData,
    rows: i32,
}

impl Spreadsheet {
    /// Create a new spreadsheet with `rows` rows and 26 columns (A–Z).
    pub fn new(rows: i32) -> Self {
        let mut data: SheetData = Vec::with_capacity(26);
        for i in 0..26 {
            let col = (b'A' + i as u8) as char;
            data.push(vec![0; rows as usize]);
        }
        Self { data, rows }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        let (col, row) = Self::parse_cell_ref(cell);
        if let Some(column) = self.data.get_mut(col) {
            if row < self.rows as usize {
                column[row] = value;
            }
        }
    }

    /// Reset a cell to zero.
    pub fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    /// Evaluate a formula like "=A1+B2" or "=5+7".
    pub fn get_value(&self, formula: String) -> i32 {
        if !formula.starts_with('=') {
            return 0;
        }

        let terms: Vec<&str> = formula.trim_start_matches('=').split('+').collect();
        terms.iter().map(|t| self.eval_term(t.trim())).sum()
    }

    /// Parse a term: either a number or a cell reference like "B3".
    fn eval_term(&self, term: &str) -> i32 {
        if let Ok(num) = term.parse::<i32>() {
            num
        } else {
            let (col, row) = Self::parse_cell_ref(term.to_string());
            self.get_cell_value(col, row)
        }
    }

    /// Convert "A1" → ("A", 0). Rows are 1-indexed externally.
    fn parse_cell_ref(cell_ref: String) -> (usize, usize) {
        let (col, row_str) = cell_ref.split_at(1);
        let col: usize = (col.chars().next().unwrap() as u8 - b'A') as usize;
        let row: usize = row_str.parse().unwrap_or(1) - 1;
        (col, row)
    }

    /// Get value of a cell if it exists, otherwise return 0.
    fn get_cell_value(&self, col: usize, row: usize) -> i32 {
        self.data
            .get(col)
            .and_then(|column| column.get(row).copied())
            .unwrap_or(0)
    }
}
