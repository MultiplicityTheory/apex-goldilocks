//! Lane layout primitives

use goldilocks::GoldilocksField;

pub struct Lane {
    pub index: usize,
    pub buffer: Vec<GoldilocksField>,
}

impl Lane {
    pub fn new(index: usize, capacity: usize) -> Self {
        Self {
            index,
            buffer: Vec::with_capacity(capacity),
        }
    }
}
