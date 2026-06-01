use std::collections::HashMap;

pub type LaneIndex = i32;
pub type ClassID = String;

#[derive(Debug, Clone, Default)]
pub struct LaneBuffer {
    pub values: HashMap<ClassID, i64>,
}

impl LaneBuffer {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, cls: &ClassID, default: i64) -> i64 {
        *self.values.get(cls).unwrap_or(&default)
    }

    pub fn set(&mut self, cls: ClassID, value: i64) {
        self.values.insert(cls, value);
    }
}

#[derive(Debug, Clone)]
pub struct MultiClassLaneStore {
    pub active_class: ClassID,
    pub lanes: HashMap<LaneIndex, LaneBuffer>,
}

impl MultiClassLaneStore {
    pub fn new(active_class: ClassID) -> Self {
        Self {
            active_class,
            lanes: HashMap::new(),
        }
    }

    pub fn ensure(&mut self, n: LaneIndex) -> &mut LaneBuffer {
        self.lanes.entry(n).or_insert_with(LaneBuffer::new)
    }

    pub fn write(&mut self, n: LaneIndex, value: i64, cls: Option<ClassID>) {
        let active = cls.unwrap_or_else(|| self.active_class.clone());
        let buf = self.ensure(n);
        buf.set(active, value);
    }

    pub fn read(&mut self, n: LaneIndex, cls: Option<&ClassID>, default: i64) -> i64 {
        let active = match cls {
            Some(c) => c.clone(),
            None => self.active_class.clone(),
        };
        let buf = self.ensure(n);
        buf.get(&active, default)
    }

    pub fn switch_class(&mut self, new_class: ClassID) {
        self.active_class = new_class;
    }

    pub fn snapshot_class(&self, cls: Option<&ClassID>) -> HashMap<LaneIndex, i64> {
        let target_class = cls.unwrap_or(&self.active_class);
        self.lanes
            .iter()
            .map(|(&n, lb)| (n, lb.get(target_class, 0)))
            .collect()
    }

    pub fn clear_class(&mut self, cls: Option<&ClassID>) {
        let target_class = cls.unwrap_or(&self.active_class);
        for lb in self.lanes.values_mut() {
            lb.values.remove(target_class);
        }
    }
}
