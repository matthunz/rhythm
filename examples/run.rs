use embedded_time::rate::Hertz;
use rhythm::{Scheduler, Task};
use std_embedded_time::StandardClock;

fn main() {
    let tasks = [
        Task::from_frequency(Hertz::new(1), |_| {
            dbg!("a");
            Ok(())
        }),
        Task::from_frequency(Hertz::new(2), |_| {
            dbg!("b");
            Ok(())
        }),
    ];
    let clock = StandardClock::default();
    let mut scheduler = Scheduler::new(tasks, clock);

    scheduler.run(&mut ())
}
