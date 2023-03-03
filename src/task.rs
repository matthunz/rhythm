use embedded_time::{duration::Milliseconds, fixed_point::FixedPoint, rate::Rate, Clock, Instant};

pub struct Task<T, E, C: Clock> {
    f: fn(&T) -> Result<(), E>,
    pub priority: u8,
    pub period: Milliseconds<C::T>,
    pub last_run: Option<Instant<C>>,
}

impl<T, E, C: Clock> Task<T, E, C> {
    pub fn new(period: Milliseconds<C::T>, f: fn(&T) -> Result<(), E>) -> Self {
        Self {
            f,
            priority: 0,
            period,
            last_run: None,
        }
    }

    pub fn from_frequency(
        frequency: impl Rate + FixedPoint<T = C::T>,
        f: fn(&T) -> Result<(), E>,
    ) -> Self {
        Self::new(frequency.to_duration().unwrap(), f)
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
                Some(ms.integer())
            } else {
                None
            }
        } else {
            Some(now.duration_since_epoch().integer())
        }
    }

    pub fn reset(&mut self) {
        self.last_run = None;
    }

    pub fn run(&mut self, now: Instant<C>, state: &T) -> Result<(), E> {
        self.last_run = Some(now);
        (self.f)(state)
    }
}
