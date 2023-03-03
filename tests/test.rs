use embedded_time::rate::{Extensions, Hertz};
use scheduler::{Scheduler, Task};
use std_embedded_time::StandardClock;

#[test]
fn it_works() {
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

    let _: () = scheduler.run(&());
}
