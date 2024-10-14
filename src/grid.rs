pub mod error;
pub mod linear;
pub mod nested;
pub mod profile;
pub mod program;
pub mod storage;

use std::time::Instant;

use error::Error;
use profile::Profile;
use storage::Storage;

use crate::Program::{self, *};

pub struct Grid<T: Storage> {
    storage: T,
}

impl<T: Storage> Grid<T> {
    pub fn new(len_row: usize, len_col: usize) -> Self {
        Self {
            storage: Storage::new(len_row, len_col),
        }
    }

    pub fn program_sum(&self, program: &Program) -> usize {
        match program {
            Program1 => self.program1(),
            Program2 => self.program2(),
            Program3 => self.program3(),
        }
    }

    fn time_one(&self, program: &Program) -> (usize, f32) {
        let timer = Instant::now();
        let sum = self.program_sum(program);
        let time = timer.elapsed().as_secs_f32() * 1000.0;
        (sum, time)
    }

    fn time_many(&self, program: &Program, repetitions: usize) -> (Vec<usize>, Vec<f32>) {
        let mut sums = Vec::with_capacity(repetitions);
        let mut times = Vec::with_capacity(repetitions);

        for _ in 0..repetitions {
            let (sum, time) = self.time_one(program);
            sums.push(sum);
            times.push(time);
        }

        (sums, times)
    }

    pub fn profile(&self, program: &Program, repetitions: usize) -> Result<Profile, Error> {
        let (sums, times) = self.time_many(program, repetitions);

        let sums_identical = match sums.first() {
            Some(first) => sums.iter().all(|item| item == first),
            None => return Err(Error::NoValues),
        };

        match sums_identical {
            true => {
                let element_count = self.storage.len_col() * self.storage.len_row();
                let element_sum = sums[0];
                let time_mean = times.iter().sum::<f32>() / times.len() as f32;
                let time_var_sum = times
                    .iter()
                    .map(|time| {
                        let diff = time - time_mean;
                        diff * diff
                    })
                    .sum::<f32>();
                let time_var = time_var_sum / times.len() as f32;
                let time_stdev = time_var.sqrt();

                Ok(Profile {
                    program: program.clone(),
                    repetitions,
                    row_count: self.storage.len_row(),
                    col_count: self.storage.len_col(),
                    element_count,
                    element_sum,
                    time_min: times.iter().cloned().reduce(f32::min).unwrap(),
                    time_max: times.iter().cloned().reduce(f32::max).unwrap(),
                    time_mean,
                    time_stdev,
                })
            }
            false => Err(Error::ElementSumMismatch),
        }
    }

    pub fn program1(&self) -> usize {
        let len_row = self.storage.len_row();
        let len_col = self.storage.len_col();

        let mut sum = 0;

        for row in 0..len_row {
            let mut col = 0;

            while col < len_col {
                let value = self.storage.value(row, col);

                sum += value;

                col = col + 1;
            }
        }

        sum
    }

    pub fn program2(&self) -> usize {
        let len_row = self.storage.len_row();
        let len_col = self.storage.len_col();

        let mut sum = 0;

        for col in 0..len_col {
            let mut row = 0;

            while row < len_row {
                let value = self.storage.value(row, col);

                sum += value;

                row = row + 1;
            }
        }

        sum
    }

    pub fn program3(&self) -> usize {
        let len_row = self.storage.len_row();
        let len_col = self.storage.len_col();

        let mut sum = 0;

        for col in 0..len_col {
            let mut row = 0;

            while row < len_row {
                let value = self.storage.value(row, col);

                sum += value;

                row = value + 1;
            }
        }

        sum
    }
}
