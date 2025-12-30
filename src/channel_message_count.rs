/// Represents a channel and its message count.
///
/// Used to store the number of messages for a specific channel.
pub struct ChannelMessageCount {
    /// The name of the channel
    pub channel_name: String,
    /// The total number of messages from this channel
    pub message_count: i32,
}
