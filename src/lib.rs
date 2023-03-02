use embedded_time::{
    duration::{Duration, Microseconds, Milliseconds},
    fixed_point::FixedPoint,
    rate::{Hertz, Rate},
    Clock, Instant,
};

pub struct Task<C: Clock> {
    f: fn(),
    priority: u8,
    period: Milliseconds<C::T>,
    last_run: Instant<C>,
}

impl<C: Clock> Task<C> {
    pub fn ready(&self, now: Instant<C>) -> Option<C::T> {
        let elapsed: Milliseconds<C::T> = now
            .checked_duration_since(&self.last_run)
            .unwrap()
            .try_into()
            .unwrap();

        if elapsed >= self.period {
            let ms = (elapsed - self.period) / self.period.integer();
            Some(ms.integer())
        } else {
            None
        }
    }

    pub fn run(&mut self, now: Instant<C>) {
        (self.f)();
        self.last_run = now;
    }
}

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
