//! Dense square matrix used for the distance/time matrix.
//!
//! Stored as a flat row-major `Vec<f64>` rather than nested vectors for better
//! cache locality, since the local search reads it in tight loops.

#[derive(Clone)]
pub struct SquareMatrix {
    size: usize,
    data: Vec<f64>,
}

impl SquareMatrix {
    pub fn new(size: usize, value: f64) -> Self {
        Self {
            size,
            data: vec![value; size * size],
        }
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.size + j]
    }

    #[inline]
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[i * self.size + j] = value;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
