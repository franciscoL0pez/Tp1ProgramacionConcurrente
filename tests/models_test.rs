use TP0ProgramacionConcurrente::models::ChatMessage;
use TP0ProgramacionConcurrente::models::LanguajeResult;
use TP0ProgramacionConcurrente::models::ChannelResult;

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
fn test_language_result_creation() {
    let lang_result = LanguajeResult {
        language : "es".to_string(),
        count: 50,
    };
    assert_eq!(lang_result.language, "es");
    assert_eq!(lang_result.count, 50);

    }

#[test]
fn test_channel_result_creation() {
    let chan_result = ChannelResult {
        channel_name: "another_channel".to_string(),
        avg_viewers: 75.5,
    };
    assert_eq!(chan_result.channel_name, "another_channel");
    assert_eq!(chan_result.avg_viewers, 75.5);
}


