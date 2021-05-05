use is_macro::Is;
use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[must_use]
pub enum ParseMode {
    MarkdownV2,
    Markdown,
    #[serde(rename = "HTML")]
    Html,
}

/// Represents input text, notably text messages or captions, potentially with
/// `HTML` or `Markdown{,V2}` markup applied.
///
/// You probably won't need to use this type explicitly: wherever `tbot` needs
/// a `Text` instance, it takes `impl Into<Text>` as the argument. For example,
/// any string, notably `String` and `&str` (and anything that implements
/// `Into<String>`), can be used in place of `Text` if you just need a plain
/// string without _all_ **this** ~~madness~~. Here's an example:
///
/// ```no_run
/// use tbot::{prelude::*, types::message::From};
///
/// let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
/// bot.start(|context| async move {
///     let sender_name = match &context.from {
///         Some(From::User(sender)) => &sender.first_name,
///         _ => return,
///     };
///
///     let result = context
///         .send_message(format!("Hi, {}!", sender_name))
///         .call()
///         .await;
///     if let Err(error) = result {
///         dbg!(error);
///     }
/// });
/// ```
///
/// But if you _really_ want to highlight something, you can choose any of
/// `HTML`, `MarkdownV2` and `Markdown` (the latter is not recommended now
/// though). If you chose `HTML` or `MarkdownV2`, `tbot`'s got something for
/// you: [`tbot::markup`] lets you build markup using either of those,
/// automatically escaping all provided strings and ensuring that the resulting
/// string is well-formed. Here's a short example, see [that module's
/// documentation][`tbot::markup`] for more:
///
/// ```no_run
/// use tbot::{markup::{bold, markdown_v2}, prelude::*};
///
/// # let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
/// bot.start(|context| async move {
///     # let sender_name = "";
///     // ..
///
///     let result = context
///         .send_message(markdown_v2(("Hi, ", bold(sender_name), "!")))
///         .call()
///         .await;
///     if let Err(error) = result {
///         dbg!(error);
///     }
/// });
/// ```
///
/// So this is another case when you don't need to use `Text` explicitly. There
/// _are_ cases when you may need to use it directly though. For example,
/// if you want to mix markups for some reason:
///
/// ```no_run
/// use tbot::{markup::{markdown_v2, bold}, types::{message::From, parameters::Text}};
///
/// # let context: tbot::contexts::Text = panic!();
/// let response: Text = match context.from {
///     Some(From::User(sender)) => markdown_v2(("Hi, ", bold(sender.first_name), "!")).into(),
///     Some(_) => return,
///     None => "Wow, I'm in a channel!".into(),
/// };
/// ```
///
/// The latter transformation can also be replaced with
/// [`Text::with_plain`]`("Wow, I'm in a channel!")`. Likewise, there are
/// [`Text::with_markdown_v2`], [`Text::with_html`] and [`Text::with_markdown`]
/// just in case you want to use an already marked-up string. Remember that
/// you need to maintain well-formness yourself in this case, so that's why
/// `tbot` encourages using [`tbot::markup`] for generating marked-up messages.
///
/// If you look closely at the Bot API docs, you'll notice another way to
/// provide markup without all these parse modes: by providing
/// `Vec<`[`message::text::Entity`]`>`. So what about `tbot`'s support for this?
/// Well, we decided that this feature is of low importance, especially since
/// `tbot` has got you covered with a better feature here!
///
/// [`tbot::util::entities`] lets you parse messages with entities easily and
/// without having to care about all that UTF-16 stuff:
///
/// ```no_run
/// use tbot::util::entities;
///
/// # let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
/// bot.text(|context| async move {
///     for entity in entities(&context.text) {
///         dbg!(entity);
///         // an entity may a code block, or inline code, or a “semantic”
///         // entity, e.g. a username, link or just text, that may have
///         // different formatting applied to its different parts.
///     }
/// });
/// ```
///
/// What's interesting for us in our case is that
/// [`entities`][`tbot::util::entities`] returns a value that can also be used
/// with [`tbot::markup`]! Here's an example:
///
/// ```no_run
/// use tbot::{util::entities, markup::markdown_v2, prelude::*};
///
/// # let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
/// bot.text(|context| async move {
///     let entities = entities(&context.text);
///     let echo = markdown_v2(entities);
///
///     let result = context.send_message(echo).call().await;
///     if let Err(error) = result {
///         dbg!(error);
///     }
/// });
/// ```
///
/// The above is an example of echoing messages with preserving formatting!
/// And that's the basic usecase for specifying `entities` instead of
/// `parse_mode`. Yet passing `entities` directly has a downside: if you want
/// to amend the user's message that's being echoed, you have to be very careful
/// that all offsets in the entities are shifted accordingly, otherwise the
/// message will be misformatted or, worse, some offset will occur inside a
/// character. This gets worse as the offsets are reported per UTF-16: this
/// makes life easier for JavaScript people, but we Rustaceans use UTF-8, and
/// the difference between them makes stuff a bit more complicated.
///
/// [`entities`][`tbot::util::entities`], on the other hand, already handles
/// UTF-16 for you and returns a `Vec` which you can modify quite easily
/// keeping all the formatting in place. Decorating user messages is even
/// easier: [`entities`][`tbot::util::entities`] can be used just like
/// [`markup::bold`] or [`markup::link`].
///
/// That being said, if you _really_, _really_ need something like
/// `Text::with_entities`, we're accepting merge requests on our [GitLab]
/// and [GitHub] repositories.
///
/// [`tbot::markup`]: crate::markup
/// [`markup::bold`]: crate::markup::bold
/// [`markup::link`]: crate::markup::link
/// [`Text::with_plain`]: Self::with_plain
/// [`Text::with_markdown_v2`]: Self::with_markdown_v2
/// [`Text::with_markdown`]: Self::with_markdown
/// [`Text::with_html`]: Self::with_html
/// [`message::text::Entity`]: crate::types::message::text::Entity
/// [`tbot::util::entities`]: crate::util::entities::entities
/// [gitlab]: https://gitlab.com/SnejUgal/tbot
/// [github]: https://github.com/tbot-rs/tbot
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
pub struct Text {
    pub(crate) text: String,
    pub(crate) parse_mode: Option<ParseMode>,
}

impl Display for ParseMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MarkdownV2 => "MarkdownV2",
            Self::Markdown => "Markdown",
            Self::Html => "HTML",
        })
    }
}

impl Text {
    /// Constructs a `Text` instance with plain text, i.e. without any parse
    /// mode applied.
    pub fn with_plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: None,
        }
    }

    /// Constructs a new `Text` instance with text using `Markdown` markup.
    ///
    /// Note that it is your responsibility to provide well-formed `Markdown`
    /// text if you wish to use `Markdown` parse mode.
    ///
    /// Telegram Bot API implemented a newer version of Markdown markup,
    /// `MarkdownV2`, which supports more formatting options and better works
    /// with escaping. If you want to use the newer version, call
    /// [`Text::with_markdown_v2`] instead.
    ///
    /// [`Text::with_markdown_v2`]: Self::with_markdown_v2
    pub fn with_markdown(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Markdown),
        }
    }

    /// Constructs a new `Text` instance with text using `MarkdownV2` markup.
    ///
    /// Note that it is your responsibility to provide well-formed `MarkdownV2`
    /// text if you wish to use `MarkdownV2` parse mode and call this method
    /// directly. On the other hand, using [`tbot::markup::markdown_v2`] ensures
    /// that your `MarkdownV2` text is always well-formed, since it
    /// automatically escapes all strings. Its return value implements
    /// `Into<Text>`, and so can be used wherever a `Text` instance is expected.
    ///
    /// If you want to use the older `Markdown` parse mode for some reason,
    /// look into [`Text::with_markdown`] instead.
    ///
    /// [`tbot::markup::markdown_v2`]: crate::markup::markdown_v2::markdown_v2
    /// [`Text::markdown`]: Self::with_markdown
    pub fn with_markdown_v2(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::MarkdownV2),
        }
    }

    /// Constructs a new `Text` instance with text using `HTML` markup.
    ///
    /// Note that it is your responsibility to provide well-formed `HTML` text
    /// if you wish to use `HTML` parse mode and call this method directly.
    /// On the other hand, using [`tbot::markup::html`] ensures that your `HTML`
    /// text is always well-formed, since it automatically escapes all strings.
    /// Its return value implements `Into<Text>`, and so can be used wherever
    /// a `Text` instance is expected.
    ///
    /// [`tbot::markup::html`]: crate::markup::html::html
    pub fn with_html(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            parse_mode: Some(ParseMode::Html),
        }
    }

    /// Checks if no parse mode is set, i.e. this `Text` instance contains
    /// plain text.
    #[must_use]
    pub fn is_plain(&self) -> bool {
        self.parse_mode == None
    }

    /// Checks if the parse mode is set to `MarkdownV2`.
    #[must_use]
    pub fn is_markdown_v2(&self) -> bool {
        self.parse_mode == Some(ParseMode::MarkdownV2)
    }

    /// Checks if the parse mode is set to `Markdown`.
    #[must_use]
    pub fn is_markdown(&self) -> bool {
        self.parse_mode == Some(ParseMode::Markdown)
    }

    /// Checks if the parse mode is set to `Html`.
    #[must_use]
    pub fn is_html(&self) -> bool {
        self.parse_mode == Some(ParseMode::Html)
    }
}

impl<T: Into<String>> From<T> for Text {
    fn from(text: T) -> Self {
        Self::with_plain(text)
    }
}
