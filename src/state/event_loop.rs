use crate::event_loop::{EventLoop, Polling, Webhook};
use crate::{contexts, errors};
use std::{future::Future, sync::Arc};

macro_rules! handler {
    (
        $context:path,
        $(#[doc = $doc:literal])+
        $name:ident,
        if: $(#[doc = $doc_if:literal])+
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

        paste::item! {
            $(#[doc = $doc_if])+
            pub fn [<$name _if>]<H, HF, P, PF>(
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
        }
    };
}

/// A stateful event loop.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub struct StatefulEventLoop<S> {
    inner: EventLoop,
    state: Arc<S>,
}

#[allow(clippy::use_self)] // https://github.com/rust-lang/rust-clippy/issues/4143
impl<S> StatefulEventLoop<S> {
    pub(crate) fn new(inner: EventLoop, state: S) -> Self {
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
    pub fn into_stateless(self) -> EventLoop {
        self.inner
    }

    /// Turns this event loop into another with other state. Handlers added on
    /// this event loop are still kept and will receive the previous state.
    pub fn with_other_state<T>(self, other_state: T) -> StatefulEventLoop<T>
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
    pub fn polling(self) -> Polling {
        self.inner.polling()
    }

    /// Starts webhook configuration.
    ///
    /// See our [wiki] to learn how to use webhook with `tbot`.
    ///
    /// [wiki]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    pub fn webhook(self, url: &str, port: u16) -> Webhook<'_> {
        self.inner.webhook(url, port)
    }
}

impl<S> StatefulEventLoop<S>
where
    S: Send + Sync + 'static,
{
    /// Adds a new handler for a command.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> F)
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
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command_if<H, HF, P, PF>(
        &mut self,
        command: &'static str,
        predicate: P,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> PF)
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
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> F)
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
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn commands_if<Cm, H, HF, P, PF>(
        &mut self,
        commands: Cm,
        predicate: P,
        handler: H,
    ) where
        Cm: IntoIterator<Item = &'static str>,
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> PF)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> PF)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> PF)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::Text>>, Arc<S>) -> PF)
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
        H: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> PF)
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
        H: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> HF)
            + Send
            + Sync
            + 'static,
        HF: Future<Output = ()> + Send + 'static,
        P: (Fn(Arc<contexts::Command<contexts::EditedText>>, Arc<S>) -> PF)
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
        contexts::Update,
        /// Adds a new handler which is run after handling an update.
        after_update,
        if: /// Adds a new handler which is run after handling an update and
        /// if the predicate returns true.
    }

    handler! {
        contexts::Animation,
        /// Adds a new handler for animations.
        animation,
        if: /// Adds a new handler for animations which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Audio,
        /// Adds a new handler for audio.
        audio,
        if: /// Adds a new handler for audio which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Update,
        /// Adds a new handler which is run before handling an update.
        before_update,
        if: /// Adds a new handler which is run before handling an update and
        /// if the predicate returns true.
    }

    handler! {
        contexts::ChosenInline,
        /// Adds a new handler for chosen inline results.
        chosen_inline,
        if: /// Adds a new handler for chosen inline results which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Contact,
        /// Adds a new handler for contacts.
        contact,
        if: /// Adds a new handler for contacts which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::ConnectedWebsite,
        /// Adds a new handler for connected websites.
        connected_website,
        if: /// Adds a new handler for connected websites which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::CreatedGroup,
        /// Adds a new handler for created groups.
        created_group,
        if: /// Adds a new handler for created groups which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::DataCallback,
        /// Adds a new handler for data callbacks.
        data_callback,
        if: /// Adds a new handler for data callbacks which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::DeletedChatPhoto,
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo,
        if: /// Adds a new handler for deleted chat photos which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Dice,
        /// Adds a new handler for dice.
        dice,
        if: /// Adds a new handler for dice which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Document,
        /// Adds a new handler for documents.
        document,
        if: /// Adds a new handler for documents which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::EditedAnimation,
        /// Adds a new handler for edited animations.
        edited_animation,
        if: /// Adds a new handler for edited animations which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::EditedAudio,
        /// Adds a new handler for edited audio.
        edited_audio,
        if: /// Adds a new handler for edited audio which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::EditedDocument,
        /// Adds a new handler for edited documents.
        edited_document,
        if: /// Adds a new handler for edited documents which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::EditedLocation,
        /// Adds a new handler for edited locations.
        edited_location,
        if: /// Adds a new handler for edited locations which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::EditedPhoto,
        /// Adds a new handler for edited photos.
        edited_photo,
        if: /// Adds a new handler for edited photos which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::EditedText,
        /// Adds a new handler for edited text messages.
        edited_text,
        if: /// Adds a new handler for edited text messages which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::EditedVideo,
        /// Adds a new handler for edited videos.
        edited_video,
        if: /// Adds a new handler for edited videos which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::GameCallback,
        /// Adds a new handler for game callbacks.
        game_callback,
        if: /// Adds a new handler for game callbacks which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Game,
        /// Adds a new handler for game messages.
        game,
        if: /// Adds a new handler for game messages which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Inline,
        /// Adds a new handler for inline queries.
        inline,
        if: /// Adds a new handler for inline queries which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Invoice,
        /// Adds a new handler for invoices.
        invoice,
        if: /// Adds a new handler for invoices which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::LeftMember,
        /// Adds a new handler for left members.
        left_member,
        if: /// Adds a new handler for left members which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Location,
        /// Adds a new handler for locations.
        location,
        if: /// Adds a new handler for locations which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Migration,
        /// Adds a new handler for migrations.
        migration,
        if: /// Adds a new handler for migrations which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::NewChatPhoto,
        /// Adds a new handler for new chat photos.
        new_chat_photo,
        if: /// Adds a new handler for new chat photos which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::NewChatTitle,
        /// Adds a new handler for new chat titles.
        new_chat_title,
        if: /// Adds a new handler for new chat titles which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::NewMembers,
        /// Adds a new handler for new members.
        new_members,
        if: /// Adds a new handler for new members which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Passport,
        /// Adds a new handler for passport data.
        passport,
        if: /// Adds a new handler for passport data which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Payment,
        /// Adds a new handler for successful payments.
        payment,
        if: /// Adds a new handler for successful payments which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Photo,
        /// Adds a new handler for photos.
        photo,
        if: /// Adds a new handler for photos which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::PinnedMessage,
        /// Adds a new handler for pinned messages.
        pinned_message,
        if: /// Adds a new handler for pinned messages which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Poll,
        /// Adds a new handler for poll messages.
        poll,
        if: /// Adds a new handler for poll messages which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::PreCheckout,
        /// Adds a new handler for pre-checkout queries.
        pre_checkout,
        if: /// Adds a new handler for pre-checkout queries which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Shipping,
        /// Adds a new handler for shipping queries.
        shipping,
        if: /// Adds a new handler for shipping queries which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Sticker,
        /// Adds a new handler for stickers.
        sticker,
        if: /// Adds a new handler for stickers which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Text,
        /// Adds a new handler for text messages.
        text,
        if: /// Adds a new handler for text messages which is run if the
        /// predicate returns true.
    }

    handler! {
        contexts::Unhandled,
        /// Adds a new handler for unhandled updates.
        unhandled,
        if: /// Adds a new handler for unhandled updates which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::UpdatedPoll,
        /// Adds a new handler for new states of polls.
        updated_poll,
        if: /// Adds a new handler for new states of polls which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::PollAnswer,
        /// Adds a new handler for new answers in the poll.
        poll_answer,
        if: /// Adds a new handler for new answers in the poll which is run
        /// if the predicate returns true.
    }

    handler! {
        contexts::Venue,
        /// Adds a new handler for venues.
        venue,
        if: /// Adds a new handler for venues which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Video,
        /// Adds a new handler for videos.
        video,
        if: /// Adds a new handler for videos which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::VideoNote,
        /// Adds a new handler for video notes.
        video_note,
        if: /// Adds a new handler for video notes which is run if the predicate
        /// returns true.
    }

    handler! {
        contexts::Voice,
        /// Adds a new handler for voice messages.
        voice,
        if: /// Adds a new handler for voice messages which is run if the
        /// predicate returns true.
    }
}

impl<S> StatefulEventLoop<S> {
    /// Fetches the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        self.inner.fetch_username().await
    }
}
