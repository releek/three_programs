pub trait Storage {
    fn new(row_len: usize, col_len: usize) -> Self;
    fn value(&self, i: usize, j: usize) -> usize;
    fn len_row(&self) -> usize;
    fn len_col(&self) -> usize;
}
