use serde::Deserialize;

/// Represents [`EncryptedCredentials`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#encryptedcredentials
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[non_exhaustive]
pub struct Credentials {
    /// Base64-encoded JSON-serialized data required for decryption.
    pub data: String,
    /// Base64-encoded hash for data authentication.
    pub hash: String,
    /// Base64-encoded secret required for data decryption.
    pub secret: String,
}
