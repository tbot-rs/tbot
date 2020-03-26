use crate::event_loop::{EventLoop, Polling, Webhook};
use crate::{connectors::Connector, contexts, errors};
use std::{future::Future, sync::Arc};

macro_rules! handler {
    (
        #[doc = $doc:literal]
        $name:ident,
        $context:path,
    ) => {
        #[doc = $doc]
        pub fn $name<H, F>(&mut self, handler: H)
        where
            H: (Fn(Arc<$context>, Arc<S>) -> F)
                + Send
                + Sync
                + 'static,
            F: Future<Output = ()> + Send + 'static,
        {
            let state = Arc::clone(&self.state);
            self.inner
                .$name(move |context| handler(context, Arc::clone(&state)));
        }
    };
}

/// A stateful event loop.
#[allow(clippy::module_name_repetitions)]
pub struct StatefulEventLoop<C, S> {
    inner: EventLoop<C>,
    state: Arc<S>,
}

impl<C, S> StatefulEventLoop<C, S> {
    pub(crate) fn new(inner: EventLoop<C>, state: S) -> Self {
        Self {
            inner,
            state: Arc::new(state),
        }
    }

    /// Returns an `Arc` to the state.
    #[must_use]
    pub fn get_state(&self) -> Arc<S> {
        Arc::clone(&self.state)
    }

    /// Turns this event loop into a stateless one. Handlers added on this event
    /// loop are still kept.
    pub fn into_stateless(self) -> EventLoop<C> {
        self.inner
    }

    /// Turns this event loop into another with other state. Handlers added on
    /// this event loop are still kept and will receive the previous state.
    pub fn with_other_state<T>(self, other_state: T) -> StatefulEventLoop<C, T>
    where
        T: Send + Sync + 'static,
    {
        StatefulEventLoop {
            inner: self.inner,
            state: Arc::new(other_state),
        }
    }

    /// Sets the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub fn username(&mut self, username: String) {
        self.inner.username(username);
    }

    /// Starts polling configuration.
    pub fn polling(self) -> Polling<C> {
        self.inner.polling()
    }

    /// Starts webhook configuration.
    ///
    /// See our [wiki] to learn how to use webhook with `tbot`.
    ///
    /// [wiki]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    pub fn webhook(self, url: &str, port: u16) -> Webhook<'_, C> {
        self.inner.webhook(url, port)
    }
}

impl<C, S: Send + Sync + 'static> StatefulEventLoop<C, S> {
    /// Adds a new handler for a command.
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.command(command, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    /// Adds a new handler for a sequence of commands.
    pub fn commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.commands(commands, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    /// Adds a new handler for the `/start` command.
    pub fn start<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("help", handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("settings", handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.edited_command(command, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    /// Adds a new handler for a sequence of edited commands.
    pub fn edited_commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.edited_commands(commands, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    handler! {
        /// Adds a new handler which is run after handling an update.
        after_update,
        contexts::Update<C>,
    }

    handler! {
        /// Adds a new handler for animations.
        animation,
        contexts::Animation<C>,
    }

    handler! {
        /// Adds a new handler for audio.
        audio,
        contexts::Audio<C>,
    }

    handler! {
        /// Adds a new handler which is run before handling an update.
        before_update,
        contexts::Update<C>,
    }

    handler! {
        /// Adds a new handler for chosen inline results.
        chosen_inline,
        contexts::ChosenInline<C>,
    }

    handler! {
        /// Adds a new handler for contacts.
        contact,
        contexts::Contact<C>,
    }

    handler! {
        /// Adds a new handler for connected websites.
        connected_website,
        contexts::ConnectedWebsite<C>,
    }

    handler! {
        /// Adds a new handler for created groups.
        created_group,
        contexts::CreatedGroup<C>,
    }

    handler! {
        /// Adds a new handler for data callbacks.
        data_callback,
        contexts::DataCallback<C>,
    }

    handler! {
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo,
        contexts::DeletedChatPhoto<C>,
    }

    handler! {
        /// Adds a new handler for documents.
        document,
        contexts::Document<C>,
    }

    handler! {
        /// Adds a new handler for edited animations.
        edited_animation,
        contexts::EditedAnimation<C>,
    }

    handler! {
        /// Adds a new handler for edited audio.
        edited_audio,
        contexts::EditedAudio<C>,
    }

    handler! {
        /// Adds a new handler for edited documents.
        edited_document,
        contexts::EditedDocument<C>,
    }

    handler! {
        /// Adds a new handler for edited locations.
        edited_location,
        contexts::EditedLocation<C>,
    }

    handler! {
        /// Adds a new handler for edited photos.
        edited_photo,
        contexts::EditedPhoto<C>,
    }

    handler! {
        /// Adds a new handler for edited text messages.
        edited_text,
        contexts::EditedText<C>,
    }

    handler! {
        /// Adds a new handler for edited videos.
        edited_video,
        contexts::EditedVideo<C>,
    }

    handler! {
        /// Adds a new handler for game callbacks.
        game_callback,
        contexts::GameCallback<C>,
    }

    handler! {
        /// Adds a new handler for game messages.
        game,
        contexts::Game<C>,
    }

    handler! {
        /// Adds a new handler for inline queries.
        inline,
        contexts::Inline<C>,
    }

    handler! {
        /// Adds a new handler for invoices.
        invoice,
        contexts::Invoice<C>,
    }

    handler! {
        /// Adds a new handler for left members.
        left_member,
        contexts::LeftMember<C>,
    }

    handler! {
        /// Adds a new handler for locations.
        location,
        contexts::Location<C>,
    }

    handler! {
        /// Adds a new handler for migrations.
        migration,
        contexts::Migration<C>,
    }

    handler! {
        /// Adds a new handler for new chat photos.
        new_chat_photo,
        contexts::NewChatPhoto<C>,
    }

    handler! {
        /// Adds a new handler for new chat titles.
        new_chat_title,
        contexts::NewChatTitle<C>,
    }

    handler! {
        /// Adds a new handler for new members.
        new_members,
        contexts::NewMembers<C>,
    }

    handler! {
        /// Adds a new handler for passport data.
        passport,
        contexts::Passport<C>,
    }

    handler! {
        /// Adds a new handler for successful payments.
        payment,
        contexts::Payment<C>,
    }

    handler! {
        /// Adds a new handler for photos.
        photo,
        contexts::Photo<C>,
    }

    handler! {
        /// Adds a new handler for pinned messages.
        pinned_message,
        contexts::PinnedMessage<C>,
    }

    handler! {
        /// Adds a new handler for poll messages.
        poll,
        contexts::Poll<C>,
    }

    handler! {
        /// Adds a new handler for pre-checkout queries.
        pre_checkout,
        contexts::PreCheckout<C>,
    }

    handler! {
        /// Adds a new handler for shipping queries.
        shipping,
        contexts::Shipping<C>,
    }

    handler! {
        /// Adds a new handler for stickers.
        sticker,
        contexts::Sticker<C>,
    }

    handler! {
        /// Adds a new handler for text messages.
        text,
        contexts::Text<C>,
    }

    handler! {
        /// Adds a new handler for unhandled updates.
        unhandled,
        contexts::Unhandled<C>,
    }

    handler! {
        /// Adds a new handler for new states of polls.
        updated_poll,
        contexts::UpdatedPoll<C>,
    }

    handler! {
        /// Adds a new handler for new answers in the poll.
        poll_answer,
        contexts::PollAnswer<C>,
    }

    handler! {
        /// Adds a new handler for venues.
        venue,
        contexts::Venue<C>,
    }

    handler! {
        /// Adds a new handler for videos.
        video,
        contexts::Video<C>,
    }

    handler! {
        /// Adds a new handler for video notes.
        video_note,
        contexts::VideoNote<C>,
    }

    handler! {
        /// Adds a new handler for voice messages.
        voice,
        contexts::Voice<C>,
    }
}

impl<C: Connector, S> StatefulEventLoop<C, S> {
    /// Fetches the bot's username.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        self.inner.fetch_username().await
    }
}
