use super::Polling;
use crate::event_loop::{EventLoop, Webhook};
use crate::{contexts, errors};
use std::{future::Future, sync::Arc};

macro_rules! handlers {
    (
        $(
            $(#[doc = $doc:literal])+
            $name:ident: $context:path,
        )+
    ) => {
        $(
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
        )+
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
    // https://github.com/rust-lang/rust-clippy/issues/4979
    #[allow(clippy::missing_const_for_fn)]
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
    pub fn polling(self) -> Polling<S> {
        Polling::new(self.inner, Arc::clone(&self.state))
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
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.command(command, move |context| {
            handler(context, Arc::clone(&state))
        });
    }

    /// Adds a new handler for a command and sets its description.
    ///
    /// Note that commands such as `/command@username` will be completely
    /// ignored unless you configure the event loop with your bot's username
    /// with either [`username`] or [`fetch_username`].
    ///
    /// [`username`]: #method.username
    /// [`fetch_username`]: #method.fetch_username
    pub fn command_with_description<H, F>(
        &mut self,
        command: &'static str,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let state = Arc::clone(&self.state);
        self.inner.command_with_description(
            command,
            description,
            move |context| handler(context, Arc::clone(&state)),
        );
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
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
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
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/start` command and sets its description.
    pub fn start_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("start", description, handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("help", handler);
    }

    /// Adds a new handler for the `/help` command and sets its description.
    pub fn help_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("help", description, handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("settings", handler);
    }

    /// Adds a new handler for the `/settings` command and sets its description.
    pub fn settings_with_description<H, F>(
        &mut self,
        description: &'static str,
        handler: H,
    ) where
        H: (Fn(Arc<contexts::Command>, Arc<S>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_with_description("settings", description, handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::EditedCommand>, Arc<S>) -> F)
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
        H: (Fn(Arc<contexts::EditedCommand>, Arc<S>) -> F)
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

    handlers! {
        /// Adds a new handler for animations.
        animation: contexts::Animation,
        /// Adds a new handler for audio.
        audio: contexts::Audio,
        /// Adds a new handler for chosen inline results.
        chosen_inline: contexts::ChosenInline,
        /// Adds a new handler for contacts.
        contact: contexts::Contact,
        /// Adds a new handler for connected websites.
        connected_website: contexts::ConnectedWebsite,
        /// Adds a new handler for created groups.
        created_group: contexts::CreatedGroup,
        /// Adds a new handler for data callbacks from chat messages.
        message_data_callback: contexts::MessageDataCallback,
        /// Adds a new handler for data callbacks from inline messages.
        inline_data_callback: contexts::InlineDataCallback,
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo: contexts::DeletedChatPhoto,
        /// Adds a new handler for dice.
        dice: contexts::Dice,
        /// Adds a new handler for documents.
        document: contexts::Document,
        /// Adds a new handler for edited animations.
        edited_animation: contexts::EditedAnimation,
        /// Adds a new handler for edited audio.
        edited_audio: contexts::EditedAudio,
        /// Adds a new handler for edited documents.
        edited_document: contexts::EditedDocument,
        /// Adds a new handler for edited locations.
        edited_location: contexts::EditedLocation,
        /// Adds a new handler for edited photos.
        edited_photo: contexts::EditedPhoto,
        /// Adds a new handler for edited text messages.
        edited_text: contexts::EditedText,
        /// Adds a new handler for edited videos.
        edited_video: contexts::EditedVideo,
        /// Adds a new handler for game callbacks from chat messages.
        message_game_callback: contexts::MessageGameCallback,
        /// Adds a new handler for game callbacks from inline messages.
        inline_game_callback: contexts::InlineGameCallback,
        /// Adds a new handler for game messages.
        game: contexts::Game,
        /// Adds a new handler for inline queries.
        inline: contexts::Inline,
        /// Adds a new handler for invoices.
        invoice: contexts::Invoice,
        /// Adds a new handler for left members.
        left_member: contexts::LeftMember,
        /// Adds a new handler for locations.
        location: contexts::Location,
        /// Adds a new handler for migrations.
        migration: contexts::Migration,
        /// Adds a new handler for new chat photos.
        new_chat_photo: contexts::NewChatPhoto,
        /// Adds a new handler for new chat titles.
        new_chat_title: contexts::NewChatTitle,
        /// Adds a new handler for new members.
        new_members: contexts::NewMembers,
        /// Adds a new handler for passport data.
        passport: contexts::Passport,
        /// Adds a new handler for successful payments.
        payment: contexts::Payment,
        /// Adds a new handler for photos.
        photo: contexts::Photo,
        /// Adds a new handler for pinned messages.
        pinned_message: contexts::PinnedMessage,
        /// Adds a new handler for poll messages.
        poll: contexts::Poll,
        /// Adds a new handler for pre-checkout queries.
        pre_checkout: contexts::PreCheckout,
        /// Adds a new handler for proximity alerts.
        proximity_alert: contexts::ProximityAlert,
        /// Adds a new handler for shipping queries.
        shipping: contexts::Shipping,
        /// Adds a new handler for stickers.
        sticker: contexts::Sticker,
        /// Adds a new handler for text messages.
        text: contexts::Text,
        /// Adds a new handler for unhandled updates.
        unhandled: contexts::Unhandled,
        /// Adds a new handler for new states of polls.
        updated_poll: contexts::UpdatedPoll,
        /// Adds a new handler for new answers in the poll.
        poll_answer: contexts::PollAnswer,
        /// Adds a new handler for venues.
        venue: contexts::Venue,
        /// Adds a new handler for videos.
        video: contexts::Video,
        /// Adds a new handler for video notes.
        video_note: contexts::VideoNote,
        /// Adds a new handler for voice messages.
        voice: contexts::Voice,
    }
}

impl<S> StatefulEventLoop<S> {
    /// Fetches the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    // `StatefulEventLoop` can be constructed only if `S: Send + Sync`
    #[allow(clippy::future_not_send)]
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        self.inner.fetch_username().await
    }
}
