/// Represents what markup the text is in.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ParseMode {
    /// The message will be parsed as Markdown.
    Markdown,
    #[serde(rename = "HTML")]
    /// The message will be parsed as HTML.
    Html,
}
