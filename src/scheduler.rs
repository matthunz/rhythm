use crate::Task;
use embedded_time::{Clock, Instant};

pub struct Scheduler<const N: usize, C: Clock> {
    tasks: [Task<C>; N],
    clock: C,
}

impl<const N: usize, C: Clock> Scheduler<N, C> {
    pub fn new(tasks: [Task<C>; N], clock: C) -> Self {
        Self { tasks, clock }
    }

    pub fn next_task(&mut self, now: Instant<C>) -> Option<&mut Task<C>> {
        let mut next: Option<(&mut Task<C>, C::T)> = None;

        for task in &mut self.tasks {
            if let Some(missed_cycles) = task.ready(now) {
                if let Some((next_task, next_missed_cycles)) = &mut next {
                    if task.priority > next_task.priority || missed_cycles > *next_missed_cycles {
                        next = Some((task, missed_cycles));
                    }
                } else {
                    next = Some((task, missed_cycles));
                }
            }
        }

        next.map(|(task, _missed_cycles)| task)
    }

    pub fn run(&mut self) {
        loop {
            let now = self.clock.try_now().unwrap();

            if let Some(task) = self.next_task(now) {
                task.run(now);
            }
        }
    }
}
