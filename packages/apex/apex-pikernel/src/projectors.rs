use ndarray::Array1;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ProjectorFamily {
    pub name: String,
    pub indexsets: Vec<Vec<usize>>,
    pub dim: usize,
}

impl ProjectorFamily {
    pub fn new(indexsets: Vec<Vec<usize>>, name: String) -> Self {
        let mut all_indices = HashSet::new();
        for set in &indexsets {
            for &idx in set {
                if !all_indices.insert(idx) {
                    panic!("Index sets for {} are not disjoint", name);
                }
            }
        }
        let dim = all_indices.len();
        ProjectorFamily { name, indexsets, dim }
    }

    pub fn block_indices(&self, block_id: usize) -> &[usize] {
        &self.indexsets[block_id]
    }

    pub fn project(&self, x: &Array1<i128>, block_id: usize) -> Array1<i128> {
        let indices = &self.indexsets[block_id];
        let mut result = Array1::zeros(indices.len());
        for (i, &idx) in indices.iter().enumerate() {
            result[i] = x[idx];
        }
        result
    }

    pub fn embed(&self, coeffs: &Array1<i128>, block_id: usize, ambient_dim: usize) -> Array1<i128> {
        let mut result = Array1::zeros(ambient_dim);
        let indices = &self.indexsets[block_id];
        for (i, &idx) in indices.iter().enumerate() {
            result[idx] = coeffs[i];
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct PiIndexGrid {
    pub families: Vec<ProjectorFamily>,
    pub dim: usize,
    pub atoms: Vec<(Vec<usize>, Vec<usize>)>, // (pi_id as tuple, indices)
    pub map: HashMap<Vec<usize>, Vec<usize>>,
    pub piids: Vec<Vec<usize>>,
}

impl PiIndexGrid {
    pub fn new(families: Vec<ProjectorFamily>) -> Self {
        let dim = families[0].dim;
        assert!(families.iter().all(|f| f.dim == dim), "All families must have same ambient dimension");

        let mut map = HashMap::new();
        let mut piids = Vec::new();
        let mut atoms = Vec::new();

        // Build grid (Cartesian product of blocks)
        let block_counts: Vec<usize> = families.iter().map(|f| f.indexsets.len()).collect();
        
        // This is a bit tricky in Rust, but we can iterate over combinations
        // Using a simple recursive approach
        Self::build_atoms(&families, 0, &mut Vec::new(), &mut map, &mut piids, &mut atoms);

        PiIndexGrid { families, dim, atoms, map, piids }
    }

    fn build_atoms(
        families: &[ProjectorFamily],
        depth: usize,
        current_pi: &mut Vec<usize>,
        map: &mut HashMap<Vec<usize>, Vec<usize>>,
        piids: &mut Vec<Vec<usize>>,
        atoms: &mut Vec<(Vec<usize>, Vec<usize>)>
    ) {
        if depth == families.len() {
            // Compute intersection of coordinate sets
            let mut intersection: Option<HashSet<usize>> = None;
            for (i, &block_id) in current_pi.iter().enumerate() {
                let set: HashSet<usize> = families[i].indexsets[block_id].iter().cloned().collect();
                intersection = match intersection {
                    None => Some(set),
                    Some(s) => Some(s.intersection(&set).cloned().collect()),
                };
            }

            if let Some(set) = intersection {
                if !set.is_empty() {
                    let indices: Vec<usize> = {
                        let mut v: Vec<usize> = set.into_iter().collect();
                        v.sort();
                        v
                    };
                    map.insert(current_pi.clone(), indices.clone());
                    piids.push(current_pi.clone());
                    atoms.push((current_pi.clone(), indices));
                }
            }
            return;
        }

        for i in 0..families[depth].indexsets.len() {
            current_pi.push(i);
            Self::build_atoms(families, depth + 1, current_pi, map, piids, atoms);
            current_pi.pop();
        }
    }

    pub fn indices(&self, pi_id: &[usize]) -> &[usize] {
        &self.map[pi_id]
    }
}
