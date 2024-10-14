use super::storage::Storage;

pub struct Linear {
    row_len: usize,
    col_len: usize,
    values: Vec<usize>,
}

impl Storage for Linear {
    fn new(row_len: usize, col_len: usize) -> Self {
        let mut values = vec![1; row_len * col_len];

        for row in 0..row_len {
            for col in 0..col_len {
                // Fill each cell with its own row index
                values[col + row * col_len] = row;
            }
        }

        Self {
            row_len,
            col_len,
            values,
        }
    }

    fn value(&self, i: usize, j: usize) -> usize {
        self.values[j + i * self.col_len]
    }

    fn len_row(&self) -> usize {
        self.row_len
    }

    fn len_col(&self) -> usize {
        self.col_len
    }
}

// pub type Linear = Grid<Vec<Vec<usize>>>;

// impl Grid<Vec<Vec<usize>>> {
//     pub fn new_linear(row_len: usize, col_len: usize) -> Self {
//         let mut values = vec![vec![1; col_len]; row_len];

//         for row in 0..row_len {
//             for col in 0..col_len {
//                 // Fill each cell with its own row index
//                 values[row][col] = row;
//             }
//         }

//         Self {
//             row_len,
//             col_len,
//             values,
//         }
//     }

//     pub fn value(&self, i: usize, j: usize) -> usize {
//         self.values[i][j]
//     }
// }

// use std::time::Instant;

// use crate::Program::{self, *};

// pub struct Linear {
//     row_len: usize,
//     col_len: usize,
//     values: Vec<usize>,
// }

// impl Linear {
//     pub fn new(row_len: usize, col_len: usize) -> Self {
//         let mut values = vec![1; row_len * col_len];

//         for row in 0..row_len {
//             for col in 0..col_len {
//                 // Fill each cell with its own row index
//                 values[col + row * col_len] = row;
//             }
//         }

//         Self {
//             row_len,
//             col_len,
//             values,
//         }
//     }

//     pub fn row_len(&self) -> usize {
//         self.row_len
//     }

//     pub fn col_len(&self) -> usize {
//         self.col_len
//     }

//     pub fn value(&self, i: usize, j: usize) -> usize {
//         self.values[j + i * self.col_len]
//     }

//     pub fn program_sum(&self, program: Program) -> (Program, usize) {
//         let sum = match program {
//             Program1 => self.program1(),
//             Program2 => self.program2(),
//             Program3 => self.program3(),
//         };

//         (program, sum)
//     }

//     pub fn program_sum_time(&self, program: Program) -> (Program, usize, f32) {
//         let timer = Instant::now();
//         let (program, sum) = self.program_sum(program);
//         let time = timer.elapsed().as_secs_f32() * 1000.0;
//         (program, sum, time)
//     }

//     pub fn program1(&self) -> usize {
//         let mut sum = 0;

//         // Iterate rows
//         for row in 0..self.row_len {
//             let mut col = 0;

//             // Iterate 'columns' - use while loop (rather than for) to make it comparable to program3
//             while col < self.col_len {
//                 // Extract value
//                 let value = self.value(row, col);

//                 // Accumulate
//                 sum += value;

//                 // Increment column index by 1
//                 col = col + 1;
//             }
//         }

//         sum
//     }

//     pub fn program2(&self) -> usize {
//         let mut sum = 0;

//         // Iterate 'columns'
//         for col in 0..self.col_len {
//             let mut row = 0;

//             // Iterate 'rows' - use while loop (rather than for) to make it comparable to program3
//             while row < self.row_len {
//                 // Extract value
//                 let value = self.value(row, col);

//                 // Accumulate
//                 sum += value;

//                 // Increment row index by 1
//                 row = row + 1;
//             }
//         }

//         sum
//     }

//     pub fn program3(&self) -> usize {
//         let mut sum = 0;

//         // Iterate 'columns'
//         for col in 0..self.col_len {
//             let mut row = 0;

//             // Iterate 'rows' - while loop necessary as we manually set the next index by fetching it from the data set
//             while row < self.row_len {
//                 // Extract value
//                 let value = self.value(row, col);

//                 // Accumulate
//                 sum += value;

//                 // Increment row index - the data set is structured such that each cell contains its own 'row' index, +1 for the next 'row'
//                 row = value + 1;
//             }
//         }

//         sum
//     }
// }
