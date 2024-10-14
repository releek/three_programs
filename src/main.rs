mod grid;

use std::fs::OpenOptions;
use std::io::Write;

use grid::program::Program::{self, *};
use grid::{linear::Linear, Grid};

fn main() {
    let repetitions = 10; // How many times to repeat the benchmark for each grid size
    let step_size = 1000; // The size increase of the side length of the grid with each iteration 10 -> 10*10, 20*20 etc
    let element_limit = 1600000000; // Total element limit for the grid - too big and the limits of the RAM will be exceeded

    by_len(step_size, repetitions, element_limit);
    by_program(step_size, repetitions, element_limit);
}

fn by_program(step_size: usize, repetitions: usize, element_limit: usize) {
    let mut file = OpenOptions::new()
        .append(true) // Append new data to the file
        .create(true) // Create the file if it doesn't exist
        .open("by_program.txt")
        .expect("Could not create file");

    let programs = [Program1, Program2, Program3];

    for program in programs {
        let mut len = step_size;

        while len * len <= element_limit {
            let grid = Grid::<Linear>::new(len, len);
            let profile = grid.profile(&program, repetitions);
            println!("{:?}", profile);
            writeln!(file, "{:?}", profile).expect("Could not write to file");
            len += step_size;
        }
    }
}

fn by_len(step_size: usize, repetitions: usize, element_limit: usize) {
    let mut file = OpenOptions::new()
        .append(true) // Append new data to the file
        .create(true) // Create the file if it doesn't exist
        .open("by_len.txt")
        .expect("Could not create file");

    let programs = [Program1, Program2, Program3];

    let mut len = step_size;

    while len * len <= element_limit {
        for program in &programs {
            let grid = Grid::<Linear>::new(len, len);
            let profile = grid.profile(&program, repetitions);
            println!("{:?}", profile);
            writeln!(file, "{:?}", profile).expect("Could not write to file");
        }
        len += step_size;
    }
}
