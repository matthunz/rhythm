use embedded_time::{duration::Milliseconds, fixed_point::FixedPoint, Clock, Instant, rate::Hertz};

pub struct Task<C: Clock> {
    f: fn(),
    pub priority: u8,
    pub period: Milliseconds<C::T>,
    pub last_run: Option<Instant<C>>,
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
