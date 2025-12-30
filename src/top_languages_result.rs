use crate::language_message_count::LanguageMessageCount;

/// Result structure for the top languages by viewer range transformation.
///
/// Contains the top 5 languages with the most messages for a specific viewer range.
pub struct TopLanguagesByViewerRangeResult {
    /// The viewer count range (e.g., "0-100", "101-500", "501-1000", "1000+")
    pub viewer_range: String,
    /// Vector of the top languages ordered by message count (descending)
    pub top_languages: Vec<LanguageMessageCount>,
}
