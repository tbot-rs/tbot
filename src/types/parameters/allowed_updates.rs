// https://github.com/rust-lang/rust/issues/78835
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! allowed_updates {
    ($($update_kind:ident,)+) => {
        use serde::{
            ser::{Serializer, SerializeSeq},
            de::{
                Deserialize, Deserializer, Error, SeqAccess, Visitor,
            },
            Serialize,
        };

        /// Represents a set of allowed updates.
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        #[must_use]
        pub struct AllowedUpdates {
            $($update_kind: bool,)+
        }

        impl AllowedUpdates {
            /// Constructs `AllowedUpdates` with none of the updates allowed to be received.
            pub const fn none() -> Self {
                Self {
                    $($update_kind: false,)+
                }
            }

            /// Constructs `AllowedUpdates` with all updates allowed to be received.
            pub const fn all() -> Self {
                Self {
                    $($update_kind: true,)+
                }
            }

            $(
                doc_comment!{
                    concat!(
                        "Configures if the `", stringify!($update_kind), "` update is allowed to be received.",
                    ),
                    pub const fn $update_kind(mut self, is_allowed: bool) -> Self {
                        self.$update_kind = is_allowed;
                        self
                    }
                }
            )+
        }

        impl Serialize for AllowedUpdates {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_seq(None)?;

                $(
                    if self.$update_kind {
                        seq.serialize_element(stringify!($update_kind))?;
                    }
                )+

                seq.end()
            }
        }

        struct AllowedUpdatesVisitor;

        impl<'v> Visitor<'v> for AllowedUpdatesVisitor {
            type Value = AllowedUpdates;

            fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(fmt, "an AllowedUpdates sequence")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'v>,
            {
                let mut allowed_updates = AllowedUpdates::none();

                while let Some(update_kind) = seq.next_element()? {
                    match update_kind {
                        $(
                            stringify!($update_kind) => {
                                allowed_updates = allowed_updates.$update_kind(true);
                            }
                        )+
                        _ => {
                            return Err(Error::unknown_variant(
                                update_kind,
                                &[$(stringify!($update_kind),)+]
                            ))
                        }
                    }
                }

                Ok(allowed_updates)
            }
        }

        impl<'de> Deserialize<'de> for AllowedUpdates {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_seq(
                    AllowedUpdatesVisitor,
                )
            }
        }
    }
}

allowed_updates! {
    message,
    edited_message,
    channel_post,
    edited_channel_post,
    inline_query,
    chosen_inline_result,
    callback_query,
    shipping_query,
    pre_checkout_query,
    poll,
    poll_answer,
    my_chat_member,
    chat_member,
}

impl Default for AllowedUpdates {
    /// Constructs `AllowedUpdates` with all updates except `chat_member` allowed to be received.
    fn default() -> Self {
        Self::all().chat_member(false)
    }
}
