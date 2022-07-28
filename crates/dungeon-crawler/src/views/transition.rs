use crate::config;
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct Transition<T> {
    pub state: T,
    new_state: T,
    last_change: Option<Instant>,
}

impl<T> Transition<T>
where
    T: Clone,
    T: PartialEq,
{
    pub fn new(state: T) -> Self {
        Self {
            state: state.clone(),
            new_state: state.clone(),
            last_change: None,
        }
    }

    pub fn tick(&mut self) -> f32 {
        if self.new_state != self.state {
            match self.last_change {
                Some(time) => {
                    if time.elapsed().as_secs_f32() > config::UI_TRANSITION_TIME * 0.5 {
                        self.state = self.new_state.clone();
                    }
                }
                None => {
                    self.last_change = Some(Instant::now());
                }
            };
        } else if let Some(time) = self.last_change {
            if time.elapsed().as_secs_f32() > config::UI_TRANSITION_TIME {
                self.last_change = None;
            }
        }

        match self.last_change {
            Some(time) => {
                let e = time.elapsed().as_secs_f32() / config::UI_TRANSITION_TIME;
                (smootherstep(0.0, 1.0, e) * 2.0 - 1.0).abs().min(1.0)
            }
            None => 1.0,
        }
    }

    pub fn set(&mut self, new_state: T) {
        self.new_state = new_state;
    }
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let x = ((x - edge0) / (edge1 - edge0)).min(1.0).max(0.0);
    return x * x * x * (x * (x * 6.0 - 15.0) + 10.0);
}
