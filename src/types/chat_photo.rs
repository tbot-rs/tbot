#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}