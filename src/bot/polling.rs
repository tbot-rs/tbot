use super::*;

/// Configures polling and runs it.
pub struct Polling<'a> {
    bot: Bot,
    limit: Option<u8>,
    timeout: Option<u64>,
    allowed_updates: Option<&'a [types::Updates]>,
    poll_interval: u64,
}

impl<'a> Polling<'a> {
    #[must_use]
    pub(crate) fn new(bot: Bot) -> Self {
        Self {
            bot,
            limit: None,
            timeout: None,
            allowed_updates: None,
            poll_interval: 25,
        }
    }

    /// Configures the limit of requested updates per request.
    #[must_use]
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Configures the timeout for long polling.
    #[must_use]
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Configures which updates you'd like to listen to.
    #[must_use]
    pub fn allowed_updates(
        mut self,
        allowed_updates: &'a [types::Updates],
    ) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Configures the minimal interval between making requests.
    #[must_use]
    pub fn poll_interval(mut self, poll_interval: u64) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Starts the event loop.
    pub fn start(self) -> ! {
        let interval = Duration::from_millis(self.poll_interval);
        let last_offset = Arc::new(Mutex::new(None));
        let mut last_send_timestamp;
        let bot = Arc::new(self.bot);

        loop {
            // Couldn't find a better way to use bot in both map and map_err
            let on_ok = bot.clone();
            let on_err = bot.clone();
            let new_offset = last_offset.clone();

            last_send_timestamp = Instant::now();

            let request = GetUpdates::new(
                &bot.token,
                *last_offset.lock().unwrap(),
                self.limit,
                self.timeout,
                self.allowed_updates,
            )
                .into_future()
                .map(move |updates| {
                if let Some(update) = updates.last() {
                    *new_offset.lock().unwrap() = Some(update.update_id + 1);
                    }
                })
                .map_err(move |error| on_err.handle_polling_error(&error));

            tokio::run(request);

            let next_timestamp = last_send_timestamp + interval;
            let now = Instant::now();

            if next_timestamp > now {
                std::thread::sleep(next_timestamp - now);
            }
        }
    }
}
