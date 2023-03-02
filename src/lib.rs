use embedded_time::{duration::Milliseconds, fixed_point::FixedPoint, Clock, Instant};

pub struct Task<C: Clock> {
    f: fn(),
    priority: u8,
    period: Milliseconds<C::T>,
    last_run: Option<Instant<C>>,
}

impl<C: Clock> Task<C> {
    pub fn new(period: Milliseconds<C::T>, f: fn()) -> Self {
        Self {
            f,
            priority: 0,
            period,
            last_run: None,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn ready(&self, now: Instant<C>) -> Option<C::T> {
        if let Some(last_run) = self.last_run {
            let elapsed: Milliseconds<C::T> = now
                .checked_duration_since(&last_run)
                .unwrap()
                .try_into()
                .unwrap();

            if elapsed >= self.period {
                let ms = (elapsed - self.period) / self.period.integer();
                return Some(ms.integer());
            }
        }

        None
    }

    pub fn reset(&mut self) {
        self.last_run = None;
    }

    pub fn run(&mut self, now: Instant<C>) {
        (self.f)();
        self.last_run = Some(now);
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
