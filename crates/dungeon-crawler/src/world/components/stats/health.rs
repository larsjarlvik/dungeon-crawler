use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum HealthChangeType {
    Once,
    #[allow(dead_code)]
    Forever,
    OverTime(Duration),
}

#[derive(Clone, Debug)]
pub struct HealthChange {
    pub amount: f32,
    pub change_type: HealthChangeType,
    pub start: Instant,
}

impl HealthChange {
    pub fn new(amount: f32, change_type: HealthChangeType) -> Self {
        Self {
            amount,
            change_type,
            start: Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Health {
    current: f32,
    pub last_change: Instant,
    pub changes: Vec<HealthChange>,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            changes: vec![],
            last_change: Instant::now(),
        }
    }

    pub fn set(&mut self, value: f32) {
        if value != self.current {
            self.current = value;
            self.last_change = Instant::now();
        }
    }

    pub fn get(&self) -> f32 {
        self.current
    }
}
