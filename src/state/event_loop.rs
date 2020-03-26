use crate::event_loop::{EventLoop, Polling, Webhook};
use crate::{connectors::Connector, contexts, errors};
use std::{future::Future, sync::Arc};

macro_rules! handler {
    (
        $context:path,
        $(#[doc = $doc:literal])+
        $name:ident,
        $(#[doc = $doc_if:literal])+
        $name_if:ident,
    ) => {
        $(#[doc = $doc])+
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

        $(#[doc = $doc_if])+
        pub fn $name_if<H, HF, P, PF>(
            &mut self,
            predicate: P,
            handler: H,
        ) where
            H: (Fn(Arc<$context>, Arc<S>) -> HF)
                + Send
                + Sync
                + 'static,
            HF: Future<Output = ()> + Send + 'static,
            P: (Fn(Arc<$context>, Arc<S>) -> PF)
                + Send
                + Sync
                + 'static,
            PF: Future<Output = bool> + Send + 'static,
        {
            let predicate = Arc::new(predicate);
            let handler = Arc::new(handler);
            self.$name(move |context, state| {
                let predicate = Arc::clone(&predicate);
                let handler = Arc::clone(&handler);
                async move {
                    if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                        handler(context, state).await
                    }
                }
            });
        }
    };
}

/// A stateful event loop.
#[allow(clippy::module_name_repetitions)]
pub struct StatefulEventLoop<C, S> {
    inner: EventLoop<C>,
    state: Arc<S>,
}

#[allow(clippy::use_self)] // https://github.com/rust-lang/rust-clippy/issues/4143
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
    #[allow(clippy::missing_const_for_fn)] // https://github.com/rust-lang/rust-clippy/issues/4979
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

impl<C, S> StatefulEventLoop<C, S>
where
    C: Connector,
    S: Send + Sync + 'static,
{
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

    /// Adds a new handler for a command which is run if the predicate
    /// returns true.
    pub fn command_if<H, HF, P, PF>(
        &mut self,
        command: &'static str,
        predicate: P,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.command(command, move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
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

    /// Adds a new handler for a sequence of commands which is run
    /// if the predicate returns true.
    pub fn commands_if<Cm, H, HF, P, PF>(
        &mut self,
        commands: Cm,
        predicate: P,
        handler: H,
    ) where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.commands(commands, move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
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

    /// Adds a new handler for the `/start` command which is run
    /// if the predicate returns true.
    pub fn start_if<H, HF, P, PF>(&mut self, predicate: P, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.command("start", move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
        });
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

    /// Adds a new handler for the `/help` command which is run if the predicate
    /// returns true.
    pub fn help_if<H, HF, P, PF>(&mut self, predicate: P, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.command("help", move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
        });
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

    /// Adds a new handler for the `/settings` command which is run
    /// if the predicate returns true.
    pub fn settings_if<H, HF, P, PF>(&mut self, predicate: P, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.command("settings", move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
        });
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

    /// Adds a new handler for an edited command which is run if the predicate
    /// returns true.
    pub fn edited_command_if<H, HF, P, PF>(
        &mut self,
        command: &'static str,
        predicate: P,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.edited_command(command, move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
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

    /// Adds a new handler for a sequence of edited commands which is run
    /// if the predicate returns true.
    pub fn edited_commands_if<Cm, H, HF, P, PF>(
        &mut self,
        commands: Cm,
        predicate: P,
        handler: H,
    ) where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>, Arc<S>) -> PF)
            + Send
            + Sync
            + 'static,
        PF: Future<Output = bool> + Send + 'static,
    {
        let predicate = Arc::new(predicate);
        let handler = Arc::new(handler);
        self.edited_commands(commands, move |context, state| {
            let predicate = Arc::clone(&predicate);
            let handler = Arc::clone(&handler);
            async move {
                if predicate(Arc::clone(&context), Arc::clone(&state)).await {
                    handler(context, state).await
                }
            }
        });
    }

    handler! {
        contexts::Update<C>,
        /// Adds a new handler which is run after handling an update.
        after_update,
        /// Adds a new handler which is run after handling an update and
        /// if the predicate returns true.
        after_update_if,
    }

    handler! {
        contexts::Animation<C>,
        /// Adds a new handler for animations.
        animation,
        /// Adds a new handler for animations which is run if the predicate
        /// returns true.
        animation_if,
    }

    handler! {
        contexts::Audio<C>,
        /// Adds a new handler for audio.
        audio,
        /// Adds a new handler for audio which is run if the predicate
        /// returns true.
        audio_if,
    }

    handler! {
        contexts::Update<C>,
        /// Adds a new handler which is run before handling an update.
        before_update,
        /// Adds a new handler which is run before handling an update and
        /// if the predicate returns true.
        before_update_if,
    }

    handler! {
        contexts::ChosenInline<C>,
        /// Adds a new handler for chosen inline results.
        chosen_inline,
        /// Adds a new handler for chosen inline results which is run
        /// if the predicate returns true.
        chosen_inline_if,
    }

    handler! {
        contexts::Contact<C>,
        /// Adds a new handler for contacts.
        contact,
        /// Adds a new handler for contacts which is run if the predicate
        /// returns true.
        contact_if,
    }

    handler! {
        contexts::ConnectedWebsite<C>,
        /// Adds a new handler for connected websites.
        connected_website,
        /// Adds a new handler for connected websites which is run
        /// if the predicate returns true.
        connected_website_if,
    }

    handler! {
        contexts::CreatedGroup<C>,
        /// Adds a new handler for created groups.
        created_group,
        /// Adds a new handler for created groups which is run if the predicate
        /// returns true.
        created_group_if,
    }

    handler! {
        contexts::DataCallback<C>,
        /// Adds a new handler for data callbacks.
        data_callback,
        /// Adds a new handler for data callbacks which is run if the predicate
        /// returns true.
        data_callback_if,
    }

    handler! {
        contexts::DeletedChatPhoto<C>,
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo,
        /// Adds a new handler for deleted chat photos which is run
        /// if the predicate returns true.
        deleted_chat_photo_if,
    }

    handler! {
        contexts::Document<C>,
        /// Adds a new handler for documents.
        document,
        /// Adds a new handler for documents which is run if the predicate
        /// returns true.
        document_if,
    }

    handler! {
        contexts::EditedAnimation<C>,
        /// Adds a new handler for edited animations.
        edited_animation,
        /// Adds a new handler for edited animations which is run
        /// if the predicate returns true.
        edited_animation_if,
    }

    handler! {
        contexts::EditedAudio<C>,
        /// Adds a new handler for edited audio.
        edited_audio,
        /// Adds a new handler for edited audio which is run if the predicate
        /// returns true.
        edited_audio_if,
    }

    handler! {
        contexts::EditedDocument<C>,
        /// Adds a new handler for edited documents.
        edited_document,
        /// Adds a new handler for edited documents which is run
        /// if the predicate returns true.
        edited_document_if,
    }

    handler! {
        contexts::EditedLocation<C>,
        /// Adds a new handler for edited locations.
        edited_location,
        /// Adds a new handler for edited locations which is run
        /// if the predicate returns true.
        edited_location_if,
    }

    handler! {
        contexts::EditedPhoto<C>,
        /// Adds a new handler for edited photos.
        edited_photo,
        /// Adds a new handler for edited photos which is run if the predicate
        /// returns true.
        edited_photo_if,
    }

    handler! {
        contexts::EditedText<C>,
        /// Adds a new handler for edited text messages.
        edited_text,
        /// Adds a new handler for edited text messages which is run
        /// if the predicate returns true.
        edited_text_if,
    }

    handler! {
        contexts::EditedVideo<C>,
        /// Adds a new handler for edited videos.
        edited_video,
        /// Adds a new handler for edited videos which is run if the predicate
        /// returns true.
        edited_video_if,
    }

    handler! {
        contexts::GameCallback<C>,
        /// Adds a new handler for game callbacks.
        game_callback,
        /// Adds a new handler for game callbacks which is run if the predicate
        /// returns true.
        game_callback_if,
    }

    handler! {
        contexts::Game<C>,
        /// Adds a new handler for game messages.
        game,
        /// Adds a new handler for game messages which is run if the predicate
        /// returns true.
        game_if,
    }

    handler! {
        contexts::Inline<C>,
        /// Adds a new handler for inline queries.
        inline,
        /// Adds a new handler for inline queries which is run if the predicate
        /// returns true.
        inline_if,
    }

    handler! {
        contexts::Invoice<C>,
        /// Adds a new handler for invoices.
        invoice,
        /// Adds a new handler for invoices which is run if the predicate
        /// returns true.
        invoice_if,
    }

    handler! {
        contexts::LeftMember<C>,
        /// Adds a new handler for left members.
        left_member,
        /// Adds a new handler for left members which is run if the predicate
        /// returns true.
        left_member_if,
    }

    handler! {
        contexts::Location<C>,
        /// Adds a new handler for locations.
        location,
        /// Adds a new handler for locations which is run if the predicate
        /// returns true.
        location_if,
    }

    handler! {
        contexts::Migration<C>,
        /// Adds a new handler for migrations.
        migration,
        /// Adds a new handler for migrations which is run if the predicate
        /// returns true.
        migration_if,
    }

    handler! {
        contexts::NewChatPhoto<C>,
        /// Adds a new handler for new chat photos.
        new_chat_photo,
        /// Adds a new handler for new chat photos which is run if the predicate
        /// returns true.
        new_chat_photo_if,
    }

    handler! {
        contexts::NewChatTitle<C>,
        /// Adds a new handler for new chat titles.
        new_chat_title,
        /// Adds a new handler for new chat titles which is run if the predicate
        /// returns true.
        new_chat_title_if,
    }

    handler! {
        contexts::NewMembers<C>,
        /// Adds a new handler for new members.
        new_members,
        /// Adds a new handler for new members which is run if the predicate
        /// returns true.
        new_members_if,
    }

    handler! {
        contexts::Passport<C>,
        /// Adds a new handler for passport data.
        passport,
        /// Adds a new handler for passport data which is run if the predicate
        /// returns true.
        passport_if,
    }

    handler! {
        contexts::Payment<C>,
        /// Adds a new handler for successful payments.
        payment,
        /// Adds a new handler for successful payments which is run
        /// if the predicate returns true.
        payment_if,
    }

    handler! {
        contexts::Photo<C>,
        /// Adds a new handler for photos.
        photo,
        /// Adds a new handler for photos which is run if the predicate
        /// returns true.
        photo_if,
    }

    handler! {
        contexts::PinnedMessage<C>,
        /// Adds a new handler for pinned messages.
        pinned_message,
        /// Adds a new handler for pinned messages which is run if the predicate
        /// returns true.
        pinned_message_if,
    }

    handler! {
        contexts::Poll<C>,
        /// Adds a new handler for poll messages.
        poll,
        /// Adds a new handler for poll messages which is run if the predicate
        /// returns true.
        poll_if,
    }

    handler! {
        contexts::PreCheckout<C>,
        /// Adds a new handler for pre-checkout queries.
        pre_checkout,
        /// Adds a new handler for pre-checkout queries which is run
        /// if the predicate returns true.
        pre_checkout_if,
    }

    handler! {
        contexts::Shipping<C>,
        /// Adds a new handler for shipping queries.
        shipping,
        /// Adds a new handler for shipping queries which is run
        /// if the predicate returns true.
        shipping_if,
    }

    handler! {
        contexts::Sticker<C>,
        /// Adds a new handler for stickers.
        sticker,
        /// Adds a new handler for stickers which is run if the predicate
        /// returns true.
        sticker_if,
    }

    handler! {
        contexts::Text<C>,
        /// Adds a new handler for text messages.
        text,
        /// Adds a new handler for text messages which is run if the predicate
        /// returns true.
        text_if,
    }

    handler! {
        contexts::Unhandled<C>,
        /// Adds a new handler for unhandled updates.
        unhandled,
        /// Adds a new handler for unhandled updates which is run
        /// if the predicate returns true.
        unhandled_if,
    }

    handler! {
        contexts::UpdatedPoll<C>,
        /// Adds a new handler for new states of polls.
        updated_poll,
        /// Adds a new handler for new states of polls which is run
        /// if the predicate returns true.
        updated_poll_if,
    }

    handler! {
        contexts::PollAnswer<C>,
        /// Adds a new handler for new answers in the poll.
        poll_answer,
        /// Adds a new handler for new answers in the poll which is run
        /// if the predicate returns true.
        poll_answer_if,
    }

    handler! {
        contexts::Venue<C>,
        /// Adds a new handler for venues.
        venue,
        /// Adds a new handler for venues which is run if the predicate
        /// returns true.
        venue_if,
    }

    handler! {
        contexts::Video<C>,
        /// Adds a new handler for videos.
        video,
        /// Adds a new handler for videos which is run if the predicate
        /// returns true.
        video_if,
    }

    handler! {
        contexts::VideoNote<C>,
        /// Adds a new handler for video notes.
        video_note,
        /// Adds a new handler for video notes which is run if the predicate
        /// returns true.
        video_note_if,
    }

    handler! {
        contexts::Voice<C>,
        /// Adds a new handler for voice messages.
        voice,
        /// Adds a new handler for voice messages which is run if the predicate
        /// returns true.
        voice_if,
    }
}

impl<C: Connector, S> StatefulEventLoop<C, S> {
    /// Fetches the bot's username.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        self.inner.fetch_username().await
    }
}
