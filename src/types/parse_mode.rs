/// Represents what markup the text is in.
#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ParseMode {
    Markdown,
    #[serde(rename = "HTML")]
    Html,
}
