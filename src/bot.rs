use super::*;
use crate::event_loop::EventLoop;
use std::sync::Arc;

mod mock_bot;
mod token;

pub use {mock_bot::*, token::*};

/// Provides methods to call API methods.
///
/// A `Bot` implements the [`Methods`] trait which provides handy methods
/// on the struct to call API methods:
///
/// ```no_run
/// use tbot::prelude::*;
///
/// let bot = tbot::bot!("BOT_TOKEN");
///
/// let me = bot
///     .get_me()
///     .into_future()
///     .map(|me| {
///         dbg!(me);
///     })
///     .map_err(|err| {
///         dbg!(err);
///     });
///
/// tbot::run(me);
/// ```
///
/// [polling]: #method.polling
/// [webhooks]: #method.webhook
/// [`text`]: #method.text
/// [`MockBot`]: ./struct.MockBot.html
/// [`Bot::mock`]: #method.mock
/// [`Methods`]: ./methods/trait.Methods.html
pub struct Bot {
    pub(crate) token: Token,
    #[cfg(feature = "proxy")]
    pub(crate) proxy: Option<proxy::Proxy>,
}

impl Bot {
    /// Constructs a new `Bot`.
    pub fn new(token: Token) -> Self {
        Self {
            token,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Constructs a new `Bot`, extracting the token from the environment at
    /// _runtime_.
    ///
    /// If you need to extract the token at _compile time_, use [`bot!`].
    ///
    /// [`bot!`]: ./macro.bot.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tbot::Bot;
    ///
    /// let mut bot = Bot::from_env("BOT_TOKEN");
    ///
    /// let me = bot
    ///     .get_me()
    ///     .into_future()
    ///     .map(|me| {
    ///         dbg!(me);
    ///     })
    ///     .map_err(|err| {
    ///         dbg!(err);
    ///     });
    ///
    /// tbot::run(me);
    /// ```
    pub fn from_env(env_var: &'static str) -> Self {
        let token = std::env::var(env_var).unwrap_or_else(|_| {
            panic!("\n[tbot] Bot's token in {} was not specified\n", env_var)
        });

        Self::new(Token::new(token))
    }

    /// Sets a proxy through which requests to Telegram will be sent.
    #[cfg(feature = "proxy")]
    pub fn proxy(&mut self, proxy: proxy::Proxy) {
        self.proxy = Some(proxy);
    }

    /// Creates a new `MockBot` inheriting the token from this bot.
    pub fn mock(&self) -> MockBot {
        MockBot::new(
            self.token.clone(),
            #[cfg(feature = "proxy")]
            self.proxy.clone(),
        )
    }

    /// Constructs an `EventLoop`.
    pub fn event_loop(self) -> EventLoop {
        EventLoop::new(self)
    }
}

impl crate::Sealed for Bot {}

impl Methods<'_> for Bot {
    fn token(&self) -> Token {
        self.token.clone()
    }

    #[cfg(feature = "proxy")]
    fn get_proxy(&self) -> Option<proxy::Proxy> {
        self.proxy.clone()
    }
}

/// Constructs a new [`Bot`], extracting the token from the environment at
/// _compile time_.
///
/// If you need to extract the token at _runtime_, use [`Bot::from_env`].
///
/// [`Bot`]: ./struct.Bot.html
/// [`Bot::from_env`]: ./struct.Bot.html#method.from_env
///
/// # Example
///
/// ```
/// let mut bot = tbot::bot!("BOT_TOKEN");
///
/// let me = bot
///     .get_me()
///     .into_future()
///     .map(|me| {
///         dbg!(me);
///     })
///     .map_err(|err| {
///         dbg!(err);
///     });
///
/// tbot::run(me);
/// ```
#[macro_export]
macro_rules! bot {
    ($var:literal) => {{
        let token = env!($var).to_string();
        $crate::Bot::new($crate::Token::new(token))
    }};
    ($var:literal,) => {
        $crate::bot!($var)
    };
    () => {
        compile_error!("the macro must be invoked as `bot!(\"<VAR_NAME>\")`")
    };
    ($($x:tt)+) => {
        $crate::bot!()
    };
}
