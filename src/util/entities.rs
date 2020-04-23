//! A utility for parsing message entities.

use crate::types::{message, User};
use message::text::{Entity as RawEntity, EntityKind as RawEntityKind};

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
    const fn plain(value: String) -> Self {
        Self {
            value,
            is_bold: false,
            is_italic: false,
            is_strikethrough: false,
            is_underline: false,
        }
    }

    const fn from_state(value: String, state: &FormattingState) -> Self {
        Self {
            value,
            is_bold: state.is_bold,
            is_italic: state.is_italic,
            is_strikethrough: state.is_strikethrough,
            is_underline: state.id_underline,
        }
    }
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

/// Represents a semantic entity.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SemanticEntity<'a> {
    /// The semantic meaning.
    kind: Option<Kind<'a>>,
    /// A `Vec` of formatted strings.
    value: Vec<FormattedText>,
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

#[derive(Debug)]
enum TokenKind {
    End,
    Start,
}

#[derive(Debug)]
struct Token<'a> {
    priority: i8,
    kind: &'a RawEntityKind,
    position: usize,
    token_kind: TokenKind,
}

struct FormattingState {
    is_bold: bool,
    is_italic: bool,
    is_strikethrough: bool,
    id_underline: bool,
}

fn tokenize<'a>(entities: &'a [RawEntity]) -> Vec<Token<'a>> {
    let mut tokens = Vec::with_capacity(entities.len() * 2);

    entities.iter().for_each(|entity| match &entity.kind {
        RawEntityKind::Bold
        | RawEntityKind::Italic
        | RawEntityKind::Strikethrough
        | RawEntityKind::Underline => {
            tokens.push(Token {
                priority: 2,
                position: entity.offset,
                kind: &entity.kind,
                token_kind: TokenKind::Start,
            });
            tokens.push(Token {
                priority: 1,
                position: entity.offset + entity.length,
                kind: &entity.kind,
                token_kind: TokenKind::End,
            });
        }
        kind => {
            tokens.push(Token {
                priority: 3,
                position: entity.offset,
                kind,
                token_kind: TokenKind::Start,
            });
            tokens.push(Token {
                priority: 0,
                position: entity.offset + entity.length,
                kind,
                token_kind: TokenKind::End,
            });
        }
    });

    tokens.sort_by(|a, b| {
        a.position
            .cmp(&b.position)
            .then_with(|| a.priority.cmp(&b.priority))
    });

    tokens
}

fn convert_kind(kind: &RawEntityKind) -> Kind {
    match kind {
        RawEntityKind::Mention => Kind::Mention,
        RawEntityKind::Hashtag => Kind::Hashtag,
        RawEntityKind::Cashtag => Kind::Cashtag,
        RawEntityKind::BotCommand => Kind::BotCommand,
        RawEntityKind::Url => Kind::Url,
        RawEntityKind::Email => Kind::Email,
        RawEntityKind::PhoneNumber => Kind::PhoneNumber,
        RawEntityKind::TextLink(link) => Kind::TextLink(link),
        RawEntityKind::TextMention(user) => Kind::TextMention(user),
        _ => unreachable!(
            "[tbot] Entity parser failed to convert this entity kind into a \
             semantic entity: {:#?}. Please file a bug on our GitLab.",
            kind
        ),
    }
}

/// Parses the message's entities.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn entities(text: &message::Text) -> Vec<Entity> {
    let tokens = tokenize(&text.entities);

    let mut state = FormattingState {
        is_bold: false,
        id_underline: false,
        is_italic: false,
        is_strikethrough: false,
    };

    let text = text.value.encode_utf16().collect::<Vec<_>>();
    let mut entities = Vec::new();

    let mut last_start: Option<usize> = None;
    let mut last_semantic: Option<SemanticEntity> = None;

    if tokens.is_empty() {
        let semantic = SemanticEntity {
            kind: None,
            value: vec![FormattedText::plain(String::from_utf16_lossy(&text))],
        };

        entities.push(Entity::Semantic(semantic));
    } else if tokens[0].position != 0 {
        let text = String::from_utf16_lossy(&text[..tokens[0].position]);

        last_semantic = Some(SemanticEntity {
            kind: None,
            value: vec![FormattedText::plain(text)],
        });
    }

    tokens.into_iter().for_each(
        |Token {
             position,
             kind,
             token_kind,
             ..
         }| {
            match (token_kind, kind) {
                (TokenKind::Start, RawEntityKind::Bold) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));

                        last_start = Some(position);
                    }

                    state.is_bold = true;

                    if last_start.is_none() {
                        last_start = Some(position);
                    }
                }
                (TokenKind::Start, RawEntityKind::Italic) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));

                        last_start = Some(position);
                    }

                    state.is_italic = true;

                    if last_start.is_none() {
                        last_start = Some(position);
                    }
                }
                (TokenKind::Start, RawEntityKind::Strikethrough) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));

                        last_start = Some(position);
                    }

                    state.is_strikethrough = true;

                    if last_start.is_none() {
                        last_start = Some(position);
                    }
                }
                (TokenKind::Start, RawEntityKind::Underline) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));

                        last_start = Some(position);
                    }

                    state.id_underline = true;

                    if last_start.is_none() {
                        last_start = Some(position);
                    }
                }
                (TokenKind::Start, RawEntityKind::Pre(_))
                | (TokenKind::Start, RawEntityKind::Code) => {
                    if let Some(semantic) = last_semantic.take() {
                        entities.push(Entity::Semantic(semantic));
                    }

                    last_start = Some(position);
                    last_semantic = None;
                }
                (TokenKind::Start, kind) => {
                    last_start = Some(position);

                    if let Some(semantic) = last_semantic.take() {
                        entities.push(Entity::Semantic(semantic));
                    }

                    last_semantic = Some(SemanticEntity {
                        kind: Some(convert_kind(kind)),
                        value: Vec::new(),
                    });
                }
                (TokenKind::End, RawEntityKind::Bold) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));
                    }

                    last_start = Some(position);
                    state.is_bold = false;
                }
                (TokenKind::End, RawEntityKind::Italic) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));
                    }

                    last_start = Some(position);
                    state.is_italic = false;
                }
                (TokenKind::End, RawEntityKind::Strikethrough) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));
                    }

                    last_start = Some(position);
                    state.is_strikethrough = false;
                }
                (TokenKind::End, RawEntityKind::Underline) => {
                    if let Some(start) = last_start.filter(|x| *x != position) {
                        let semantic = last_semantic.get_or_insert_with(|| {
                            SemanticEntity {
                                kind: None,
                                value: Vec::with_capacity(1),
                            }
                        });

                        semantic.value.push(FormattedText::from_state(
                            String::from_utf16_lossy(&text[start..position]),
                            &state,
                        ));
                    }

                    last_start = Some(position);
                    state.id_underline = false;
                }
                (TokenKind::End, RawEntityKind::Code) => {
                    let text = String::from_utf16_lossy(
                        &text[last_start.unwrap()..position],
                    );
                    entities.push(Entity::Code(text));

                    last_start = Some(position);
                }
                (TokenKind::End, RawEntityKind::Pre(language)) => {
                    entities.push(Entity::Pre {
                        language: language.as_deref(),
                        value: String::from_utf16_lossy(
                            &text[last_start.unwrap()..position],
                        ),
                    });

                    last_start = Some(position);
                }
                (TokenKind::End, _) => {
                    let mut semantic = last_semantic.take().unwrap();
                    semantic.value.push(FormattedText::from_state(
                        String::from_utf16_lossy(
                            &text[last_start.unwrap()..position],
                        ),
                        &state,
                    ));

                    entities.push(Entity::Semantic(semantic));

                    last_start = Some(position);
                    last_semantic = None;
                }
            }
        },
    );

    if let Some(start) = last_start.filter(|x| *x != text.len()) {
        let semantic = last_semantic.get_or_insert_with(|| SemanticEntity {
            kind: None,
            value: Vec::with_capacity(1),
        });

        let text = String::from_utf16_lossy(&text[start..]);
        semantic.value.push(FormattedText::plain(text));
    }

    if let Some(semantic) = last_semantic.take() {
        entities.push(Entity::Semantic(semantic));
    }

    entities
}
