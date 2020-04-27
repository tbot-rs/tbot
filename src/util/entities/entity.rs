use super::FormattingState;
use crate::{
    markup::{
        self, bold, code_block, html, inline_code, italic, link, markdown_v2,
        mention, strikethrough, underline,
    },
    types::User,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
/// Represents a string with formatting options.
pub struct FormattedText {
    /// The text.
    pub value: String,
    /// `true` if bold is applied to this string.
    pub is_bold: bool,
    /// `true` if italic is applied to this string.
    pub is_italic: bool,
    /// `true` if strikethrough is applied to this string.
    pub is_strikethrough: bool,
    /// `true` if underline is applied to this string.
    pub is_underline: bool,
}

impl FormattedText {
    pub(crate) const fn plain(value: String) -> Self {
        Self {
            value,
            is_bold: false,
            is_italic: false,
            is_strikethrough: false,
            is_underline: false,
        }
    }

    pub(crate) const fn from_state(
        value: String,
        state: &FormattingState,
    ) -> Self {
        Self {
            value,
            is_bold: state.is_bold,
            is_italic: state.is_italic,
            is_strikethrough: state.is_strikethrough,
            is_underline: state.id_underline,
        }
    }
}

/// Represents the semantic meaning of the entity.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Kind<'a> {
    /// A mention.
    Mention,
    /// A hashtag.
    Hashtag,
    /// A cashtag (e.g. `$TBOT`).
    Cashtag,
    /// A bot command.
    BotCommand,
    /// An URL.
    Url,
    /// An email.
    Email,
    /// A phone number.
    PhoneNumber,
    /// A clickable text link.
    TextLink(&'a str),
    /// A mention for users without username.
    TextMention(&'a User),
}

/// Represents a semantic entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SemanticEntity<'a> {
    /// The semantic meaning.
    pub kind: Option<Kind<'a>>,
    /// A `Vec` of formatted strings.
    pub value: Vec<FormattedText>,
}

/// Represents a parsed entity.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Entity<'a> {
    /// Inline code.
    Code(String),
    /// A code block.
    Pre {
        /// The code's programming language.
        language: Option<&'a str>,
        /// The code.
        value: String,
    },
    /// Text that may have semantic meaning.
    Semantic(SemanticEntity<'a>),
}

fn to_formattable<'a>(
    formatted: &'a [FormattedText],
) -> Vec<Box<dyn markup::Formattable + 'a>> {
    formatted
        .iter()
        .map(
            |FormattedText {
                 value,
                 is_bold,
                 is_italic,
                 is_underline,
                 is_strikethrough,
             }| {
                let mut formatted: Box<dyn markup::Formattable> =
                    Box::new(value.as_str());

                if *is_bold {
                    formatted = Box::new(bold(formatted));
                }

                if *is_italic {
                    formatted = Box::new(italic(formatted));
                }

                if *is_underline {
                    formatted = Box::new(underline(formatted));
                }

                if *is_strikethrough {
                    formatted = Box::new(strikethrough(formatted));
                }

                formatted
            },
        )
        .collect()
}

impl<'a> markdown_v2::Formattable for SemanticEntity<'a> {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self.kind {
            None
            | Some(Kind::Mention)
            | Some(Kind::Hashtag)
            | Some(Kind::Cashtag)
            | Some(Kind::BotCommand)
            | Some(Kind::Url)
            | Some(Kind::Email)
            | Some(Kind::PhoneNumber) => markdown_v2::Formattable::format(
                &to_formattable(&self.value),
                formatter,
                nesting,
            ),
            Some(Kind::TextLink(url)) => markdown_v2::Formattable::format(
                &link(to_formattable(&self.value), url),
                formatter,
                nesting,
            ),
            Some(Kind::TextMention(user)) => markdown_v2::Formattable::format(
                &mention(to_formattable(&self.value), user.id),
                formatter,
                nesting,
            ),
        }
    }
}

impl<'a> html::Formattable for SemanticEntity<'a> {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self.kind {
            None
            | Some(Kind::Mention)
            | Some(Kind::Hashtag)
            | Some(Kind::Cashtag)
            | Some(Kind::BotCommand)
            | Some(Kind::Url)
            | Some(Kind::Email)
            | Some(Kind::PhoneNumber) => html::Formattable::format(
                &to_formattable(&self.value),
                formatter,
                nesting,
            ),
            Some(Kind::TextLink(url)) => html::Formattable::format(
                &link(to_formattable(&self.value), url),
                formatter,
                nesting,
            ),
            Some(Kind::TextMention(user)) => html::Formattable::format(
                &mention(to_formattable(&self.value), user.id),
                formatter,
                nesting,
            ),
        }
    }
}

impl<'a> markdown_v2::Formattable for Entity<'a> {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self {
            Self::Code(code) => markdown_v2::Formattable::format(
                &inline_code([code.as_str()]),
                formatter,
                nesting,
            ),
            Self::Pre { language, value } => {
                let code = code_block([value.as_str()]);

                if let Some(language) = language {
                    code.language(*language);
                }

                markdown_v2::Formattable::format(&code, formatter, nesting)
            }
            Self::Semantic(semantic) => {
                markdown_v2::Formattable::format(semantic, formatter, nesting)
            }
        }
    }
}

impl<'a> html::Formattable for Entity<'a> {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self {
            Self::Code(code) => html::Formattable::format(
                &inline_code([code.as_str()]),
                formatter,
                nesting,
            ),
            Self::Pre { language, value } => {
                let code = code_block([value.as_str()]);

                if let Some(language) = language {
                    code.language(*language);
                }

                html::Formattable::format(&code, formatter, nesting)
            }
            Self::Semantic(semantic) => {
                html::Formattable::format(semantic, formatter, nesting)
            }
        }
    }
}
