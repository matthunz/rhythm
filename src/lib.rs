use std::time::{Duration, SystemTime};

pub struct Task {
    f: fn(),
    priority: u8,
    freq: u32,
    last_run: SystemTime,
}

impl Task {
    pub fn ready(&self, now: SystemTime) -> Option<u32> {
        let elapsed = now.duration_since(self.last_run).unwrap();
        let freq_duration = Duration::from_secs_f32(1.0 / self.freq as f32);

        if elapsed >= freq_duration {
            Some((elapsed.as_secs() / freq_duration.as_secs()) as u32)
        } else {
            None
        }
    }
}

pub struct Scheduler<const N: usize> {
    tasks: [Task; N],
}

impl<const N: usize> Scheduler<N> {
    pub fn new(tasks: [Task; N]) -> Self {
        Self { tasks }
    }

    pub fn next_task(&mut self) -> Option<(&Task, u32)> {
        let now = SystemTime::now();
        let mut next: Option<(&Task, u32)> = None;

        for task in &self.tasks {
            if let Some(duration) = task.ready(now) {
                if let Some((next_task, next_duration)) = next {
                    if task.priority > next_task.priority || duration > next_duration {
                        next = Some((task, duration));
                    }
                } else {
                    next = Some((task, duration));
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
