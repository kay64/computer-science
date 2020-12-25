use std::time::{Duration, Instant};
use std::thread;

trait Waiter {
    fn wait(&self, duration: Duration);
}

struct SimpleWaiter;

impl Waiter for SimpleWaiter {
    fn wait(&self, duration: Duration) {
        thread::sleep(duration);
    }
}

struct LoggingWaiter<'a> {
    original: &'a dyn Waiter,
}

impl<'a> LoggingWaiter<'a> {
    fn decorate(waiter: &'a dyn Waiter) -> LoggingWaiter {
        LoggingWaiter {
            original: waiter,
        }
    }
}

impl<'a> Waiter for LoggingWaiter<'a> {
    fn wait(&self, duration: Duration) {
        let now = Instant::now();
        self.original.wait(duration);
        println!("Waited : {:?}", now.elapsed());

    }
}

#[cfg(test)]
mod tests {
    use crate::patterns::decorator::*;

    #[test]
    fn decorator() {
        let waiter = SimpleWaiter{};

        let decorated = LoggingWaiter::decorate(&waiter);

        decorated.wait(Duration::from_secs(2));
    }
}
