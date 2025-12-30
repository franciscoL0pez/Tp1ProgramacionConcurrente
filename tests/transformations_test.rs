use TP0ProgramacionConcurrente::transformations::{
    analyze_both, top_channels_by_language, top_languages_by_viewer_range,
};
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod transformations_tests {
    use super::*;

    fn create_test_file_for_top_channels_by_language() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("top_channels_test_data.json");

        let content = r#"{"channelName": "gaming1", "language": "en", "viewerCount": 100}
{"channelName": "gaming2", "language": "en", "viewerCount": 150}
{"channelName": "gaming3", "language": "en", "viewerCount": 200}
{"channelName": "gaming4", "language": "en", "viewerCount": 250}
{"channelName": "music1", "language": "es", "viewerCount": 300}
{"channelName": "music2", "language": "es", "viewerCount": 350}
{"channelName": "tech1", "language": "fr", "viewerCount": 400}
{"channelName": "tech2", "language": "fr", "viewerCount": 450}
{"channelName": "gaming1", "language": "en", "viewerCount": 120}
{"channelName": "gaming2", "language": "en", "viewerCount": 170}
{"channelName": "music1", "language": "es", "viewerCount": 320}
{"channelName": "tech1", "language": "fr", "viewerCount": 420}"#;

        fs::write(&test_file, content).unwrap();
        (temp_dir, test_file.to_string_lossy().to_string())
    }

    fn create_test_file_for_viewer_ranges() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("viewer_ranges_test_data.json");

        let content = r#"{"channelName": "small1", "language": "en", "viewerCount": 50}
{"channelName": "small2", "language": "es", "viewerCount": 80}
{"channelName": "small3", "language": "en", "viewerCount": 95}
{"channelName": "medium1", "language": "fr", "viewerCount": 200}
{"channelName": "medium2", "language": "en", "viewerCount": 300}
{"channelName": "medium3", "language": "es", "viewerCount": 400}
{"channelName": "large1", "language": "en", "viewerCount": 800}
{"channelName": "large2", "language": "de", "viewerCount": 900}
{"channelName": "xlarge1", "language": "en", "viewerCount": 1500}
{"channelName": "xlarge2", "language": "es", "viewerCount": 2000}"#;

        fs::write(&test_file, content).unwrap();
        (temp_dir, test_file.to_string_lossy().to_string())
    }

    fn create_empty_test_file() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("empty_test_data.json");
        fs::write(&test_file, "").unwrap();
        (temp_dir, test_file.to_string_lossy().to_string())
    }

    // Tests for top_channels_by_language function
    #[test]
    fn test_top_channels_by_language_single_thread() {
        let (_temp_dir, test_file) = create_test_file_for_top_channels_by_language();
        let result = top_channels_by_language(&test_file, 1);

        assert!(result.is_ok());
        let results = result.unwrap();

        let en_result = results.iter().find(|r| r.language == "en").unwrap();
        assert_eq!(en_result.top_channels.len(), 3); // Should have top 3

        assert!(en_result.top_channels[0].message_count >= en_result.top_channels[1].message_count);
        assert!(en_result.top_channels[1].message_count >= en_result.top_channels[2].message_count);
    }

    #[test]
    fn test_top_channels_by_language_multiple_threads() {
        let (_temp_dir, test_file) = create_test_file_for_top_channels_by_language();
        let result = top_channels_by_language(&test_file, 3);

        assert!(result.is_ok());
        let results = result.unwrap();

        assert_eq!(results.len(), 3);

        let languages: Vec<&str> = results.iter().map(|r| r.language.as_str()).collect();
        assert!(languages.contains(&"en"));
        assert!(languages.contains(&"es"));
        assert!(languages.contains(&"fr"));
    }

    #[test]
    fn test_top_channels_by_language_empty_file() {
        let (_temp_dir, test_file) = create_empty_test_file();
        let result = top_channels_by_language(&test_file, 1);

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 0); // No results for empty file
    }

    #[test]
    fn test_top_channels_by_language_non_existent_file() {
        let result = top_channels_by_language("/non/existent/file.json", 1);
        assert!(result.is_err()); // Should return an error
    }

    #[test]
    fn test_top_languages_by_viewer_range_single_thread() {
        let (_temp_dir, test_file) = create_test_file_for_viewer_ranges();
        let result = top_languages_by_viewer_range(&test_file, 1);

        assert!(result.is_ok());
        let results = result.unwrap();

        assert_eq!(results.len(), 4);

        let range_0_100 = results.iter().find(|r| r.viewer_range == "0-100").unwrap();
        assert!(range_0_100.top_languages.len() <= 5);

        for i in 0..range_0_100.top_languages.len() - 1 {
            assert!(
                range_0_100.top_languages[i].message_count
                    >= range_0_100.top_languages[i + 1].message_count
            );
        }
    }

    #[test]
    fn test_top_languages_by_viewer_range_multiple_threads() {
        let (_temp_dir, test_file) = create_test_file_for_viewer_ranges();
        let result = top_languages_by_viewer_range(&test_file, 4);

        assert!(result.is_ok());
        let results = result.unwrap();

        assert_eq!(results.len(), 4);

        let ranges: Vec<&str> = results.iter().map(|r| r.viewer_range.as_str()).collect();
        assert!(ranges.contains(&"0-100"));
        assert!(ranges.contains(&"101-500"));
        assert!(ranges.contains(&"501-1000"));
        assert!(ranges.contains(&"1000+"));
    }

    #[test]
    fn test_top_languages_by_viewer_range_empty_file() {
        let (_temp_dir, test_file) = create_empty_test_file();
        let result = top_languages_by_viewer_range(&test_file, 1);

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 0); // No results for empty file
    }

    #[test]
    fn test_top_languages_by_viewer_range_non_existent_file() {
        let result = top_languages_by_viewer_range("/non/existent/file.json", 1);
        assert!(result.is_err()); // Should return an error
    }

    #[test]
    fn test_analyze_both_function() {
        let (_temp_dir, test_file) = create_test_file_for_top_channels_by_language();
        let result = analyze_both(&test_file, 2);

        assert!(result.is_ok());
        let (top_channels, top_languages) = result.unwrap();

        // Should have results for both transformations
        assert!(!top_channels.is_empty());
        assert!(!top_languages.is_empty());

        assert_eq!(top_channels.len(), 3); // en, es, fr

        assert!(top_languages.len() > 0);
    }

    #[test]
    fn test_consistency_across_thread_counts() {
        let (_temp_dir, test_file) = create_test_file_for_top_channels_by_language();

        let result_1_thread = top_channels_by_language(&test_file, 1).unwrap();
        let result_2_threads = top_channels_by_language(&test_file, 2).unwrap();
        let result_4_threads = top_channels_by_language(&test_file, 4).unwrap();

        assert_eq!(result_1_thread.len(), result_2_threads.len());
        assert_eq!(result_2_threads.len(), result_4_threads.len());

        let en_1 = result_1_thread.iter().find(|r| r.language == "en").unwrap();
        let en_2 = result_2_threads
            .iter()
            .find(|r| r.language == "en")
            .unwrap();
        let en_4 = result_4_threads
            .iter()
            .find(|r| r.language == "en")
            .unwrap();

        assert_eq!(en_1.top_channels.len(), en_2.top_channels.len());
        assert_eq!(en_2.top_channels.len(), en_4.top_channels.len());
    }

    #[test]
    fn test_large_dataset_simulation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("large_test_data.json");

        let mut content = String::new();
        let languages = ["en", "es", "fr", "de"];
        let channels = ["gaming", "music", "tech", "cooking"];

        for i in 1..=100 {
            let language = languages[(i - 1) % 4];
            let channel = format!("{}{}", channels[(i - 1) % 4], (i - 1) / 4 + 1);
            let viewer_count = i * 10;

            content.push_str(&format!(
                r#"{{"channelName": "{}", "language": "{}", "viewerCount": {}}}"#,
                channel, language, viewer_count
            ));
            if i < 100 {
                content.push('\n');
            }
        }

        fs::write(&test_file, content).unwrap();

        let result = top_channels_by_language(&test_file.to_string_lossy(), 4);
        assert!(result.is_ok());

        let results = result.unwrap();
        assert_eq!(results.len(), 4);

        for language_result in results {
            assert!(language_result.top_channels.len() <= 3);
            // Check ordering
            for i in 0..language_result.top_channels.len() - 1 {
                assert!(
                    language_result.top_channels[i].message_count
                        >= language_result.top_channels[i + 1].message_count
                );
            }
        }
    }
}
