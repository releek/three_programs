use crate::grid::program::Program;

#[derive(Debug)]
pub struct Profile {
    pub program: Program,
    pub repetitions: usize,
    pub row_count: usize,
    pub col_count: usize,
    pub element_count: usize,
    pub element_sum: usize,
    pub time_min: f32,
    pub time_max: f32,
    pub time_mean: f32,
    pub time_stdev: f32,
}
