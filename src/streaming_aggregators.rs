/// Thread-safe aggregators for streaming data processing.
///
/// This struct contains all the data structures needed to aggregate
/// streaming data across multiple threads using Arc<Mutex<>> for thread safety.
pub struct StreamingAggregators {
    /// Language to channel message counts mapping
    pub language_channel_counts: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<String, std::collections::HashMap<String, i32>>>,
    >,
    /// Viewer range to language message counts mapping
    pub range_language_counts: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<String, std::collections::HashMap<String, i32>>>,
    >,
    /// Total number of processed messages
    pub total_messages: std::sync::Arc<std::sync::Mutex<usize>>,
}
