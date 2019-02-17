use super::*;

/// Represents updates names to subscribe with Webhooks or getUpdates.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Updates {
    /// Handles messages in a chat of any kind.
    Message,
    /// Handles a message edit.
    EditedMessage,
    /// Handles a message in a channel of any kind.
    ChannelPost,
    /// Handles a channel message edit.
    EditedChannelPost,
    /// Handles inline queries (when you type your bot's username in the
    /// beginning of a message)
    InlineQuery,
    /// When subscribed for chosen inline results, handles them.
    ChosenInlineResult,
    /// Handles inline buttons clicks.
    CallbackQuery,
    /// Handles shpping query.
    ShippingQuery,
    /// Handles pre-checkout query.
    PreCheckoutQuery,
}

/// Represents different types of updates from Telegram.
#[derive(Debug, PartialEq, Clone)]
pub enum UpdateType {
    /// A new incoming message.
    Message(raw::Message),
    /// A message was edited.
    EditedMessage(raw::Message),
    /// A new channel post.
    ChannelPost(raw::Message),
    /// A channel post was edited.
    EditedChannelPost(raw::Message),
}

/// Represents an update from Telegram.
#[derive(Debug)]
pub struct Update {
    /// Update's ID.
    pub update_id: u64,
    /// Update's type.
    pub update_type: Option<UpdateType>,
}

impl<'de> serde::Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct UpdateVisitor;

        impl<'v> serde::de::Visitor<'v> for UpdateVisitor {
            type Value = Update;

            fn expecting(
                &self,
                fmt: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(fmt, "struct Update")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'v>,
            {
                let mut update_id = None;
                let mut update_type = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "update_id" => update_id = Some(map.next_value()?),
                        "message" => {
                            update_type =
                                Some(UpdateType::Message(map.next_value()?))
                        }
                        "edited_message" => {
                            update_type = Some(UpdateType::EditedMessage(
                                map.next_value()?,
                            ))
                        }
                        "channel_post" => {
                            update_type =
                                Some(UpdateType::ChannelPost(map.next_value()?))
                        }
                        "edited_channel_post" => {
                            update_type = Some(UpdateType::EditedChannelPost(
                                map.next_value()?,
                            ))
                        }
                        _ => (),
                    }
                }

                let update_id = update_id.ok_or_else(|| {
                    serde::de::Error::missing_field("update_id")
                })?;

                Ok(Update {
                    update_id,
                    update_type,
                })
            }
        }

        deserializer.deserialize_struct(
            "Duration",
            &["update_id", "update_type"],
            UpdateVisitor,
        )
    }
}
