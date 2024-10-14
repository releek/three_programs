use super::storage::Storage;

pub struct Nested {
    row_len: usize,
    col_len: usize,
    values: Vec<Vec<usize>>,
}

impl Storage for Nested {
    fn new(row_len: usize, col_len: usize) -> Self {
        let mut values = vec![vec![1; col_len]; row_len];

        for row in 0..row_len {
            for col in 0..col_len {
                // Fill each cell with its own row index
                values[row][col] = row;
            }
        }

        Self {
            row_len,
            col_len,
            values,
        }
    }

    fn value(&self, i: usize, j: usize) -> usize {
        self.values[i][j]
    }

    fn len_row(&self) -> usize {
        self.row_len
    }

    fn len_col(&self) -> usize {
        self.col_len
    }
}

