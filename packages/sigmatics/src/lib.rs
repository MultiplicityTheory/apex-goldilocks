use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Generator {
    Mark { class: usize },
    Copy { src: usize, dst: usize },
    Swap { a: usize, b: usize },
    Merge { src1: usize, src2: usize, dst: usize },
    Split { src: usize, dst1: usize, dst2: usize },
    Quote { class: usize },
    Evaluate { class: usize },
    Mirror { class: usize },
    Add { a: usize, b: usize, c: usize },
}

pub struct Canonicalizer;

impl Canonicalizer {
    pub fn simplify(mut program: Vec<Generator>) -> Vec<Generator> {
        let mut i = 0;
        while i + 1 < program.len() {
            match (&program[i], &program[i+1]) {
                (Generator::Mirror { class: c1 }, Generator::Mirror { class: c2 }) if c1 == c2 => {
                    program.remove(i);
                    program.remove(i);
                    if i > 0 { i -= 1; }
                    continue;
                }
                _ => {}
            }
            i += 1;
        }
        program
    }
}