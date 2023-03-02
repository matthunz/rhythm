use std::time::{Duration, SystemTime};

pub struct Task {
    f: fn(),
    priority: u8,
    freq: u32,
    last_run: SystemTime,
}

impl Task {
    pub fn is_ready(&self, now: SystemTime) -> bool {
        let elapsed = now.duration_since(self.last_run).unwrap();
        let freq_duration = Duration::from_secs_f32(1.0 / self.freq as f32);

        elapsed >= freq_duration
    }
}


pub struct Scheduler<const N: usize> {
    tasks: [Task; N],
}

impl<const N: usize> Scheduler<N> {
    pub fn new(tasks: [Task; N]) -> Self {
        Self { tasks }
    }

    pub fn next_task(&mut self) ->  Option<&Task> {
        let now = SystemTime::now();
        let mut next: Option<&Task> = None;

        for task in &self.tasks {
            if task.is_ready(now) {
                if let Some(next_task) = next {
                    if task.priority > next_task.priority {
                        next = Some(next_task)
                    }
                } else {
                    next = Some(task);
                }
            }
        }

        next
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
