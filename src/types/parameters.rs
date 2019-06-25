//! Types used as parameters, mainly for methods.

mod chat_id;
mod flexibility;
mod notification_state;
mod parse_mode;
mod photo;
mod requirement;
mod send_to_provider_state;
mod text;
mod updates;
mod url_visibility;
mod web_page_preview_state;

pub(crate) use parse_mode::*;
pub use {
    chat_id::*, flexibility::*, notification_state::*, photo::*,
    requirement::*, send_to_provider_state::*, text::*, updates::*,
    url_visibility::*, web_page_preview_state::*,
};
