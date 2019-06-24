use futures::{try_ready, Async, Poll, task::{current, Task}};
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
    task: Option<Task>,
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
            task: None,
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
        let last_offset = schedule.last_offset;

        let is_ready = match try_ready!(schedule.queue.poll()) {
            Some(..) => {
                let item = (last_offset, Arc::clone(&self.schedule));
                Async::Ready(Some(item))
            },
            None => {
                schedule.task = Some(current());
                Async::NotReady
            },
        };

        Ok(is_ready)
    }
}
