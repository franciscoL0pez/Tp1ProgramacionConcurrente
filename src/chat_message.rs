/// Represents a chat message from the Twitch dataset.
///
/// This struct contains the essential information from each JSON record
/// including channel name, language, and viewer count.
#[derive(serde::Deserialize)]
pub struct ChatMessage {
    /// The name of the Twitch channel
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// The language of the channel (e.g., "en", "es", "fr")
    pub language: String,
    /// The number of viewers watching the channel
    #[serde(rename = "viewerCount")]
    pub viewer_count: i32,
}
