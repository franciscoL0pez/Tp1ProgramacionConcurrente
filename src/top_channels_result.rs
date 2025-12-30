use crate::channel_message_count::ChannelMessageCount;

/// Result structure for the top channels by language transformation.
///
/// Contains the top 3 channels with the most messages for a specific language.
pub struct TopChannelsByLanguageResult {
    /// The language code (e.g., "en", "es", "fr")
    pub language: String,
    /// Vector of the top channels ordered by message count (descending)
    pub top_channels: Vec<ChannelMessageCount>,
}
