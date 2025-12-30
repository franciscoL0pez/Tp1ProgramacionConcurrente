use TP0ProgramacionConcurrente::transformations::{
    analyze_both, top_channels_by_language, top_languages_by_viewer_range,
};
use std::io::Write;
use tempfile::NamedTempFile;

#[cfg(test)]
mod streaming_tests {
    use super::*;

    fn create_streaming_test_data() -> (NamedTempFile, String) {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

        let test_data = r#"{"channelName": "gaming1", "language": "en", "viewerCount": 150}
{"channelName": "gaming2", "language": "en", "viewerCount": 200}
{"channelName": "gaming1", "language": "en", "viewerCount": 180}
{"channelName": "gaming1", "language": "en", "viewerCount": 120}
{"channelName": "music1", "language": "es", "viewerCount": 50}
{"channelName": "music2", "language": "es", "viewerCount": 300}
{"channelName": "music1", "language": "es", "viewerCount": 400}
{"channelName": "tech1", "language": "fr", "viewerCount": 1200}
{"channelName": "tech2", "language": "fr", "viewerCount": 900}
{"channelName": "cooking1", "language": "en", "viewerCount": 800}
{"channelName": "sports1", "language": "de", "viewerCount": 600}
{"channelName": "sports2", "language": "de", "viewerCount": 700}
{"channelName": "art1", "language": "it", "viewerCount": 250}
{"channelName": "news1", "language": "pt", "viewerCount": 450}"#;

        temp_file
            .write_all(test_data.as_bytes())
            .expect("Failed to write test data");

        let file_path = temp_file.path().to_str().unwrap().to_string();
        (temp_file, file_path)
    }

    #[test]
    fn test_streaming_memory_efficiency() {
        let (_temp_file, file_path) = create_streaming_test_data();

        let result = analyze_both(&file_path, 4);
        assert!(result.is_ok());

        let (top_channels, top_languages) = result.unwrap();

        assert!(!top_channels.is_empty());
        assert!(!top_languages.is_empty());

        let languages: Vec<&str> = top_channels.iter().map(|r| r.language.as_str()).collect();
        assert!(languages.contains(&"en"));
        assert!(languages.contains(&"es"));
        assert!(languages.contains(&"fr"));
    }

    #[test]
    fn test_top_channels_by_language_streaming() {
        let (_temp_file, file_path) = create_streaming_test_data();

        let result = top_channels_by_language(&file_path, 2);
        assert!(result.is_ok());

        let results = result.unwrap();

        let en_result = results.iter().find(|r| r.language == "en").unwrap();
        assert_eq!(en_result.top_channels.len(), 3); // Should have top 3

        // gaming1 should be first (3 messages)
        assert_eq!(en_result.top_channels[0].channel_name, "gaming1");
        assert_eq!(en_result.top_channels[0].message_count, 3);

        for i in 0..en_result.top_channels.len() - 1 {
            assert!(
                en_result.top_channels[i].message_count
                    >= en_result.top_channels[i + 1].message_count
            );
        }
    }

    #[test]
    fn test_top_languages_by_viewer_range_streaming() {
        let (_temp_file, file_path) = create_streaming_test_data();

        let result = top_languages_by_viewer_range(&file_path, 3);
        assert!(result.is_ok());

        let results = result.unwrap();
        assert!(!results.is_empty());

        let ranges: Vec<&str> = results.iter().map(|r| r.viewer_range.as_str()).collect();
        assert!(ranges.contains(&"101-500"));
        assert!(ranges.contains(&"501-1000"));
        assert!(ranges.contains(&"1000+"));

        for range_result in results {
            for i in 0..range_result.top_languages.len().saturating_sub(1) {
                assert!(
                    range_result.top_languages[i].message_count
                        >= range_result.top_languages[i + 1].message_count
                );
            }
        }
    }

    #[test]
    fn test_streaming_consistency_across_thread_counts() {
        let (_temp_file, file_path) = create_streaming_test_data();

        let result_1_thread = analyze_both(&file_path, 1);
        let result_2_threads = analyze_both(&file_path, 2);
        let result_4_threads = analyze_both(&file_path, 4);

        assert!(result_1_thread.is_ok());
        assert!(result_2_threads.is_ok());
        assert!(result_4_threads.is_ok());

        let (channels_1, ranges_1) = result_1_thread.unwrap();
        let (channels_2, ranges_2) = result_2_threads.unwrap();
        let (channels_4, ranges_4) = result_4_threads.unwrap();

        assert_eq!(channels_1.len(), channels_2.len());
        assert_eq!(channels_2.len(), channels_4.len());

        assert_eq!(ranges_1.len(), ranges_2.len());
        assert_eq!(ranges_2.len(), ranges_4.len());

        let en_1 = channels_1.iter().find(|r| r.language == "en").unwrap();
        let en_2 = channels_2.iter().find(|r| r.language == "en").unwrap();
        let en_4 = channels_4.iter().find(|r| r.language == "en").unwrap();

        assert_eq!(en_1.top_channels.len(), en_2.top_channels.len());
        assert_eq!(en_2.top_channels.len(), en_4.top_channels.len());

        assert_eq!(
            en_1.top_channels[0].channel_name,
            en_2.top_channels[0].channel_name
        );
        assert_eq!(
            en_2.top_channels[0].channel_name,
            en_4.top_channels[0].channel_name
        );
        assert_eq!(
            en_1.top_channels[0].message_count,
            en_2.top_channels[0].message_count
        );
        assert_eq!(
            en_2.top_channels[0].message_count,
            en_4.top_channels[0].message_count
        );
    }

    #[test]
    fn test_streaming_with_empty_file() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(b"")
            .expect("Failed to write empty data");

        let file_path = temp_file.path().to_str().unwrap();

        let result = analyze_both(file_path, 2);
        assert!(result.is_ok());

        let (top_channels, top_languages) = result.unwrap();
        assert!(top_channels.is_empty());
        assert!(top_languages.is_empty());
    }

    #[test]
    fn test_streaming_large_dataset_simulation() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");

        let mut content = String::new();
        let languages = ["en", "es", "fr", "de", "it"];
        let channels = ["gaming", "music", "tech", "cooking", "sports"];

        for i in 1..=1000 {
            let language = languages[(i - 1) % 5];
            let channel = format!("{}{}", channels[(i - 1) % 5], (i - 1) / 5 + 1);
            let viewer_count = (i * 10) % 1500; // Vary viewer counts

            content.push_str(&format!(
                r#"{{"channelName": "{}", "language": "{}", "viewerCount": {}}}"#,
                channel, language, viewer_count
            ));
            if i < 1000 {
                content.push('\n');
            }
        }

        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write test data");
        let file_path = temp_file.path().to_str().unwrap();

        let result = analyze_both(file_path, 8);
        assert!(result.is_ok());

        let (top_channels, _top_languages) = result.unwrap();

        assert_eq!(top_channels.len(), 5);

        for language_result in top_channels {
            assert_eq!(language_result.top_channels.len(), 3);

            for i in 0..language_result.top_channels.len() - 1 {
                assert!(
                    language_result.top_channels[i].message_count
                        >= language_result.top_channels[i + 1].message_count
                );
            }
        }
    }
}
