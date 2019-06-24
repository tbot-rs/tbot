use futures::{try_ready, Async, Poll, Future, task::{current, Task}};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::timer::{self, Delay};

#[derive(Debug)]
pub struct Schedule {
    pub last_offset: Option<u32>,

    last_run: Instant,
    duration: Duration,
    delay: Delay,
    has_polled: bool,
    task: Option<Task>,
}

pub struct Stream {
    schedule: Arc<Mutex<Schedule>>,
}

impl Schedule {
    pub fn new(duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            last_offset: None,
            last_run: now,
            duration,
            delay: Delay::new(now),
            has_polled: false,
            task: None,
        }
    }

    pub fn schedule_next_tick(&mut self) {
        let next_instant = self.last_run + self.duration;

        self.last_run = Instant::now();
        self.has_polled = false;
        self.delay.reset(next_instant);

        if let Some(task) = self.task.take() {
            task.notify();
        }
    }

    pub fn into_stream(self) -> Stream {
        Stream {
            schedule: Arc::new(Mutex::new(self)),
        }
    }
}

impl futures::Stream for Stream {
    type Item = (Option<u32>, Arc<Mutex<Schedule>>);
    type Error = timer::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut schedule = self.schedule.lock().unwrap();

        if schedule.has_polled {
            schedule.task = Some(current());
            return Ok(Async::NotReady);
        }

        try_ready!(schedule.delay.poll());

        schedule.has_polled = true;
        let item = (schedule.last_offset, Arc::clone(&self.schedule));

        Ok(Async::Ready(Some(item)))
    }
}
