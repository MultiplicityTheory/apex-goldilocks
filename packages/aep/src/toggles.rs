use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ToggleStreams {
    pub schedules: HashMap<i32, Vec<bool>>,
}

impl ToggleStreams {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }

    pub fn set_schedule(&mut self, n: i32, bits: Vec<bool>) {
        self.schedules.insert(n, bits);
    }

    pub fn is_on(&self, n: i32, t: i32) -> bool {
        match self.schedules.get(&n) {
            None => true,
            Some(bits) => {
                if bits.is_empty() {
                    true
                } else {
                    let idx = (t.rem_euclid(bits.len() as i32)) as usize;
                    bits[idx]
                }
            }
        }
    }
}
