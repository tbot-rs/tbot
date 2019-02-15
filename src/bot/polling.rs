use super::{methods::DeleteWebhook, *};

/// Configures polling and runs it.
///
/// To construct `Polling`, use [`Bot::polling`].
///
/// [`Bot::polling`]: ./struct.Bot.html#method.polling
#[must_use = "polling does nothing unless `start` is called"]
pub struct Polling<'a> {
    bot: Bot,
    limit: Option<u8>,
    timeout: Option<u64>,
    allowed_updates: Option<&'a [types::Updates]>,
    poll_interval: u64,
}

impl<'a> Polling<'a> {
    pub(crate) fn new(bot: Bot) -> Self {
        Self {
            bot,
            limit: None,
            timeout: None,
            allowed_updates: None,
            poll_interval: 25,
        }
    }

    /// Configures the limit of updates per request.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Configures the timeout for long polling.
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Configures which updates you'd like to listen to.
    pub fn allowed_updates(
        mut self,
        allowed_updates: &'a [types::Updates],
    ) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    /// Configures the minimal interval between making requests.
    pub fn poll_interval(mut self, poll_interval: u64) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Starts the event loop.
    pub fn start(self) -> ! {
        let error = Arc::new(Mutex::new(Ok(())));
        let handler = error.clone();

        let bot = Arc::new(self.bot);

        let delete_webhook = DeleteWebhook::new(
            &bot.token,
            #[cfg(feature = "proxy")]
            bot.proxy.clone(),
        )
        .into_future()
        .map_err(move |error| *handler.lock().unwrap() = Err(error));

        crate::run(delete_webhook);

        if let Err(error) = &*error.lock().unwrap() {
            panic!(
                "\n[tbot] error while deleting previous webhook:\n\n{:#?}\n",
                error,
            );
        }

        let interval = Duration::from_millis(self.poll_interval);
        let last_offset = Arc::new(Mutex::new(None));
        let mut last_send_timestamp;

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
                #[cfg(feature = "proxy")]
                bot.proxy.clone(),
            );

            let request = request
                .into_future()
                .map(move |updates| {
                    if let Some(update) = updates.last() {
                        *new_offset.lock().unwrap() =
                            Some(update.update_id + 1);
                    }

                    for update in updates {
                        on_ok.handle_update(update);
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
