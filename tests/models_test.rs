use TP0ProgramacionConcurrente::channel_message_count::ChannelMessageCount;
use TP0ProgramacionConcurrente::chat_message::ChatMessage;
use TP0ProgramacionConcurrente::language_message_count::LanguageMessageCount;
use TP0ProgramacionConcurrente::top_channels_result::TopChannelsByLanguageResult;
use TP0ProgramacionConcurrente::top_languages_result::TopLanguagesByViewerRangeResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage {
            channel_name: "test_channel".to_string(),
            language: "en".to_string(),
            viewer_count: 100,
        };
        assert_eq!(message.channel_name, "test_channel");
        assert_eq!(message.language, "en");
        assert_eq!(message.viewer_count, 100);
    }

    #[test]
    fn test_top_channels_by_language_result_creation() {
        let top_channels = vec![
            ChannelMessageCount {
                channel_name: "channel1".to_string(),
                message_count: 10,
            },
            ChannelMessageCount {
                channel_name: "channel2".to_string(),
                message_count: 5,
            },
        ];

        let result = TopChannelsByLanguageResult {
            language: "en".to_string(),
            top_channels,
        };

        assert_eq!(result.language, "en");
        assert_eq!(result.top_channels.len(), 2);
        assert_eq!(result.top_channels[0].channel_name, "channel1");
        assert_eq!(result.top_channels[0].message_count, 10);
    }

    #[test]
    fn test_top_languages_by_viewer_range_result_creation() {
        let top_languages = vec![
            LanguageMessageCount {
                language: "en".to_string(),
                message_count: 15,
            },
            LanguageMessageCount {
                language: "es".to_string(),
                message_count: 8,
            },
        ];

        let result = TopLanguagesByViewerRangeResult {
            viewer_range: "101-500".to_string(),
            top_languages,
        };

        assert_eq!(result.viewer_range, "101-500");
        assert_eq!(result.top_languages.len(), 2);
        assert_eq!(result.top_languages[0].language, "en");
        assert_eq!(result.top_languages[0].message_count, 15);
    }
}
