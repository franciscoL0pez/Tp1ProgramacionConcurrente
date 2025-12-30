/// Represents a language and its message count.
///
/// Used to store the number of messages for a specific language.
pub struct LanguageMessageCount {
    /// The language code (e.g., "en", "es", "fr")
    pub language: String,
    /// The total number of messages in this language
    pub message_count: i32,
}
