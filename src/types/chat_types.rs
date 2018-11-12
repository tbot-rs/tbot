#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ChatTypes {
    Private,
    Group,
    Supergroup,
    Channel,
}
