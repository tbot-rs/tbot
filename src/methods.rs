//! Telegram Bots API methods in form of structs.
//!
//! Note that structs here are somewhat raw, because they require tokens when
//! construcing. You would more like using construction methods on `Tbot` that
//! infer `token`, and methods on context structs that in addition infer things
//! like `chat_id` or `callback_query_id`.
//!
//! Though methods don't implement a trait like `Method` for reasons, they all
//! have methods `new` with varying number of parameters and `into_future` that
//! returns a `Future` which resolves with either Telegram's response or an
//! error. The design philosophy is that all required parameters are passed
//! through `new` and all optional parameters are set with a dedicated method.
//! Then `into_future` is called to get the `Future`, handling is done and the
//! future is passed to a runner. For convenience, we re-export
//! `tokio::{run, spawn}` as `tbot::{run, spawn}`.
//!
//! For example, here's how you'd call `getMe`:
//!
//! ```
//! # use tbot::prelude::*;
//! let request = tbot::methods::GetMe::new(env!("BOT_TOKEN"))
//!     .into_future()
//!     .map_err(|error| {
//!         dbg!(error);
//!     })
//!     .map(|me| {
//!         dbg!(me);
//!     });
//!
//! tbot::run(request);
//! ```

use super::*;

mod add_sticker_to_set;
mod create_new_sticker_set;
mod delete_chat_photo;
mod delete_message;
mod delete_sticker_from_set;
mod delete_webhook;
mod edit_inline_caption;
mod edit_inline_location;
mod edit_inline_media;
mod edit_inline_reply_markup;
mod edit_inline_text;
mod edit_message_caption;
mod edit_message_location;
mod edit_message_media;
mod edit_message_reply_markup;
mod edit_message_text;
mod export_chat_invite_link;
mod forward_message;
mod get_chat;
mod get_me;
mod get_sticker_set;
mod get_updates;
mod get_user_profile_photos;
mod get_webhook_info;
mod kick_chat_member;
mod leave_chat;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_poll;
mod send_sticker;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_chat_description;
mod set_chat_photo;
mod set_chat_title;
mod set_sticker_position_in_set;
mod set_webhook;
mod stop_inline_location;
mod stop_message_location;
mod stop_poll;
mod unban_chat_member;
mod unpin_chat_message;
mod upload_sticker_file;

pub use {
    add_sticker_to_set::*, create_new_sticker_set::*, delete_chat_photo::*,
    delete_message::*, delete_sticker_from_set::*, edit_inline_caption::*,
    edit_inline_location::*, edit_inline_media::*, edit_inline_reply_markup::*,
    edit_inline_text::*, edit_message_caption::*, edit_message_location::*,
    edit_message_media::*, edit_message_reply_markup::*, edit_message_text::*,
    export_chat_invite_link::*, forward_message::*, get_chat::*, get_me::*,
    get_sticker_set::*, get_user_profile_photos::*, get_webhook_info::*,
    kick_chat_member::*, leave_chat::*, pin_chat_message::*,
    promote_chat_member::*, restrict_chat_member::*, send_animation::*,
    send_audio::*, send_chat_action::*, send_contact::*, send_document::*,
    send_location::*, send_media_group::*, send_message::*, send_photo::*,
    send_poll::*, send_sticker::*, send_venue::*, send_video::*,
    send_video_note::*, send_voice::*, set_chat_description::*,
    set_chat_photo::*, set_chat_title::*, set_sticker_position_in_set::*,
    stop_inline_location::*, stop_message_location::*, stop_poll::*,
    unban_chat_member::*, unpin_chat_message::*, upload_sticker_file::*,
};

pub(crate) use {delete_webhook::*, get_updates::*, set_webhook::*};

mod methods_trait;
pub use methods_trait::*;

mod call_method;
use call_method::*;

/// An error happened during request. Different errors may happen, so this is
/// an enum representing error that may happen during request.
#[derive(Debug)]
pub enum DeliveryError {
    /// Telegram Bots API responded with an HTML page what usually means it's
    /// down.
    TelegramOutOfService,
    /// `serde_json` couldn't parse the response. Most probably, it's a bug in
    /// `tbot` that tried to parse the response into a wrong struct, what you
    /// should fill an issue for on [our GitLab repository][issues].
    ///
    /// [issues]: https://gitlab.com/snejugal/tbot/issues
    InvalidResponse(serde_json::error::Error),
    /// Some error happened during sending the request.
    NetworkError(hyper::Error),
    /// Telegram returned an error in response. That is most probably your
    /// fault.
    RequestError {
        /// Human-readable description of the error.
        description: String,
        /// Error code reflected through HTTP error codes (for example, 401).
        error_code: u16,
        /// The group moved to a supergroup.
        migrate_to_chat_id: Option<i64>,
        /// When exceeding flood control, you must wait for this amount of
        /// seconds before making another request.
        retry_after: Option<u64>,
    },
}

#[cfg(feature = "proxy")]
/// Provides the proxy method.
pub trait ProxyMethod {
    /// Configures the proxy the method will be sent via.
    fn proxy(self, proxy: proxy::Proxy) -> Self;
}
