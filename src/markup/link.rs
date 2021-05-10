use super::{html, markdown_v2, Formattable, Nesting};
use crate::types::user;
use std::{
    fmt::{self, Formatter, Write},
    ops::Deref,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Kind<T> {
    Link(T),
    Mention(user::Id),
}

/// Formats a link. Can be created with [`link`] or [`mention`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use = "formatters need to be formatted with `markdown_v2` or `html`"]
pub struct Link<T, L = &'static str> {
    text: T,
    link: Kind<L>,
}

/// Creates a link.
pub fn link<T, L>(text: T, link: L) -> Link<T, L>
where
    T: Formattable,
    L: Deref<Target = str>,
{
    Link {
        text,
        link: Kind::Link(link),
    }
}

/// Creates a mention by ID.
pub fn mention<T: Formattable>(text: T, user: user::Id) -> Link<T> {
    Link {
        text,
        link: Kind::Mention(user),
    }
}

impl<T, L> markdown_v2::Formattable for Link<T, L>
where
    T: Formattable,
    L: Deref<Target = str>,
{
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_char('[')?;
        markdown_v2::Formattable::format(&self.text, formatter, nesting)?;
        formatter.write_str("](")?;

        match &self.link {
            Kind::Link(link) => link.deref().chars().try_for_each(|x| {
                if markdown_v2::ESCAPED_LINK_CHARACTERS.contains(&x) {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })?,
            Kind::Mention(user::Id(id)) => {
                write!(formatter, "tg://user?id={}", id)?
            }
        }
        formatter.write_char(')')
    }
}

impl<T, L> html::Formattable for Link<T, L>
where
    T: Formattable,
    L: Deref<Target = str>,
{
    fn format(
        &self,
        formatter: &mut Formatter,
        nesting: Nesting,
    ) -> fmt::Result {
        formatter.write_str("<a href=\"")?;

        match &self.link {
            Kind::Link(link) => link.deref().chars().try_for_each(|x| {
                if x == '"' {
                    formatter.write_char('\\')?;
                }
                formatter.write_char(x)
            })?,
            Kind::Mention(user::Id(id)) => {
                write!(formatter, "tg://user?id={}", id)?
            }
        }

        formatter.write_str("\">")?;
        html::Formattable::format(&self.text, formatter, nesting)?;
        formatter.write_str("</a>")
    }
}
