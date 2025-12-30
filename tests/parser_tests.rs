use TP0ProgramacionConcurrente::chunk_info::ChunkInfo;
use TP0ProgramacionConcurrente::parser::{analize_file_for_chunks, parse_chunk_streaming};
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod chunk_tests {
    use super::*;

    fn create_test_file() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_data.json");

        let content = r#"{"channelName": "channel1", "language": "en", "viewerCount": 100}
{"channelName": "channel2", "language": "es", "viewerCount": 200}
{"channelName": "channel3", "language": "en", "viewerCount": 150}
{"channelName": "channel4", "language": "fr", "viewerCount": 300}
{"channelName": "channel5", "language": "es", "viewerCount": 250}
{"channelName": "channel6", "language": "en", "viewerCount": 180}
{"channelName": "channel7", "language": "de", "viewerCount": 120}
{"channelName": "channel8", "language": "fr", "viewerCount": 220}"#;

        fs::write(&test_file, content).unwrap();
        (temp_dir, test_file.to_string_lossy().to_string())
    }

    #[test]
    fn test_analize_file_chunks_single_thread() {
        let (_temp_dir, test_file) = create_test_file();
        let result = analize_file_for_chunks(&test_file, 1);

        assert!(result.is_ok());
        let chunks = result.unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].start, 0);
        assert!(chunks[0].end > 0);
        assert_eq!(chunks[0].size, chunks[0].end - chunks[0].start);
    }

    #[test]
    fn test_analize_file_chunks_multiple_threads() {
        let (_temp_dir, test_file) = create_test_file();
        let result = analize_file_for_chunks(&test_file, 3);

        assert!(result.is_ok());
        let chunks = result.unwrap();
        assert_eq!(chunks.len(), 3);

        for i in 1..chunks.len() {
            assert_eq!(chunks[i - 1].end, chunks[i].start);
        }

        assert_eq!(chunks[0].start, 0);

        let total_size: u64 = chunks.iter().map(|c| c.size).sum();
        assert!(total_size > 0);
    }

    #[test]
    fn test_parse_chunk_streaming() {
        let (_temp_dir, test_file) = create_test_file();

        let chunks = analize_file_for_chunks(&test_file, 1).unwrap();
        let chunk = &chunks[0];

        let mut messages_count = 0;
        let mut all_messages = Vec::new();

        let result = parse_chunk_streaming(&test_file, chunk, |message| {
            messages_count += 1;
            all_messages.push(message);
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), messages_count);
        assert_eq!(messages_count, 8);

        let first_message = &all_messages[0];
        assert_eq!(first_message.channel_name, "channel1");
        assert_eq!(first_message.language, "en");
        assert_eq!(first_message.viewer_count, 100);
    }
}

#[cfg(test)]
mod streaming_parser_tests {
    use super::*;

    #[test]
    fn test_parse_chunk_streaming_single_message() {
        let test_content =
            r#"{"channelName": "testchannel", "language": "en", "viewerCount": 100}"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", test_content).unwrap();

        let chunk = ChunkInfo {
            start: 0,
            end: test_content.len() as u64,
            size: test_content.len() as u64,
        };

        let mut message_count = 0;
        let mut received_message = None;

        let result = parse_chunk_streaming(temp_file.path().to_str().unwrap(), &chunk, |message| {
            message_count += 1;
            received_message = Some(message);
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(message_count, 1);

        let message = received_message.unwrap();
        assert_eq!(message.channel_name, "testchannel");
        assert_eq!(message.language, "en");
        assert_eq!(message.viewer_count, 100);
    }

    #[test]
    fn test_parse_chunk_streaming_multiple_messages() {
        let test_content = r#"{"channelName": "channel1", "language": "en", "viewerCount": 100}
{"channelName": "channel2", "language": "es", "viewerCount": 200}"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", test_content).unwrap();

        let chunk = ChunkInfo {
            start: 0,
            end: test_content.len() as u64,
            size: test_content.len() as u64,
        };

        let mut messages = Vec::new();

        let result = parse_chunk_streaming(temp_file.path().to_str().unwrap(), &chunk, |message| {
            messages.push(message);
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
        assert_eq!(messages.len(), 2);

        assert_eq!(messages[0].channel_name, "channel1");
        assert_eq!(messages[1].channel_name, "channel2");
    }

    #[test]
    fn test_parse_chunk_streaming_with_invalid_json() {
        let test_content = r#"{"channelName": "channel1", "language": "en", "viewerCount": 100}
{"invalid": "json", "missing": "fields"}
{"channelName": "channel2", "language": "es", "viewerCount": 200}"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", test_content).unwrap();

        let chunk = ChunkInfo {
            start: 0,
            end: test_content.len() as u64,
            size: test_content.len() as u64,
        };

        let mut messages = Vec::new();

        let result = parse_chunk_streaming(temp_file.path().to_str().unwrap(), &chunk, |message| {
            messages.push(message);
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2); // Only valid messages
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_parse_chunk_streaming_memory_efficiency() {
        let mut temp_file = NamedTempFile::new().unwrap();

        // Generate larger test data
        let mut content = String::new();
        for i in 1..=100 {
            content.push_str(&format!(
                r#"{{"channelName": "channel{}", "language": "en", "viewerCount": {}}}"#,
                i,
                i * 10
            ));
            if i < 100 {
                content.push('\n');
            }
        }

        write!(temp_file, "{}", content).unwrap();

        let chunk = ChunkInfo {
            start: 0,
            end: content.len() as u64,
            size: content.len() as u64,
        };

        let mut processed_count = 0;

        let result =
            parse_chunk_streaming(temp_file.path().to_str().unwrap(), &chunk, |_message| {
                processed_count += 1;
                // Each message is processed and immediately discarded
            });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
        assert_eq!(processed_count, 100);
    }
}
