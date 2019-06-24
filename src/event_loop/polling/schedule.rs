use futures::{try_ready, Async, Poll};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::timer::{self, DelayQueue};

#[derive(Debug)]
pub struct Schedule {
    pub last_offset: Option<u32>,

    last_run: Instant,
    duration: Duration,
    queue: DelayQueue<()>,
}

pub struct Stream {
    schedule: Arc<Mutex<Schedule>>,
}

impl Schedule {
    pub fn new(duration: Duration) -> Self {
        let now = Instant::now();
        let mut queue = DelayQueue::new();

        queue.insert_at((), now);

        Self {
            last_offset: None,
            last_run: now,
            duration,
            queue,
        }
    }

    pub fn schedule_next_tick(&mut self) {
        let now = Instant::now();
        let processed_for = now - self.last_run;

        let next_instant = if processed_for > self.duration {
            now
        } else {
            self.last_run + self.duration
        };

        self.queue.insert_at((), next_instant);
    }

    pub fn into_stream(self) -> Stream {
        Stream {
            schedule: Arc::new(Mutex::new(self)),
        }
    }
}

impl futures::Stream for Stream {
    type Item = Option<(Option<u32>, Arc<Mutex<Schedule>>)>;
    type Error = timer::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut schedule = self.schedule.lock().unwrap();
        let last_offset = schedule.last_offset;

        let item = try_ready!(schedule.queue.poll())
            .map(|_| (last_offset, Arc::clone(&self.schedule)));

        Ok(Async::Ready(Some(item)))
    }
}
