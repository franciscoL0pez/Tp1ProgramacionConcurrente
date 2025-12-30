use TP0ProgramacionConcurrente::transformations::{
    analyze_both, top_channels_by_language, top_languages_by_viewer_range,
};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_integration_new_transformations() {
    let test_data = r#"{"channelName": "gaming1", "language": "en", "viewerCount": 150}
{"channelName": "gaming2", "language": "en", "viewerCount": 200}
{"channelName": "gaming1", "language": "en", "viewerCount": 180}
{"channelName": "music1", "language": "es", "viewerCount": 50}
{"channelName": "music2", "language": "es", "viewerCount": 300}
{"channelName": "tech1", "language": "fr", "viewerCount": 1200}
{"channelName": "cooking1", "language": "en", "viewerCount": 800}"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(test_data.as_bytes())
        .expect("Failed to write test data");

    let file_path = temp_file.path().to_str().unwrap();

    let result = analyze_both(file_path, 2);
    assert!(result.is_ok(), "analyze_both should succeed");

    let (top_channels_results, top_languages_results) = result.unwrap();

    assert!(
        !top_channels_results.is_empty(),
        "Should have top channels results"
    );

    let languages: Vec<&str> = top_channels_results
        .iter()
        .map(|r| r.language.as_str())
        .collect();
    assert!(languages.contains(&"en"));
    assert!(languages.contains(&"es"));
    assert!(languages.contains(&"fr"));

    let en_result = top_channels_results
        .iter()
        .find(|r| r.language == "en")
        .unwrap();
    assert_eq!(en_result.top_channels.len(), 3);

    assert!(
        !top_languages_results.is_empty(),
        "Should have top languages results"
    );

    let ranges: Vec<&str> = top_languages_results
        .iter()
        .map(|r| r.viewer_range.as_str())
        .collect();
    assert!(ranges.len() > 1, "Should have multiple viewer ranges");
}

#[test]
fn test_integration_top_channels_by_language_only() {
    let test_data = r#"{"channelName": "stream1", "language": "en", "viewerCount": 100}
{"channelName": "stream2", "language": "en", "viewerCount": 200}
{"channelName": "stream3", "language": "es", "viewerCount": 300}
{"channelName": "stream1", "language": "en", "viewerCount": 150}
{"channelName": "stream4", "language": "es", "viewerCount": 250}"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(test_data.as_bytes())
        .expect("Failed to write test data");

    let file_path = temp_file.path().to_str().unwrap();

    let result = top_channels_by_language(file_path, 1);
    assert!(result.is_ok(), "top_channels_by_language should succeed");

    let results = result.unwrap();
    assert_eq!(results.len(), 2);

    for language_result in results {
        assert!(!language_result.top_channels.is_empty());

        for i in 0..language_result.top_channels.len() - 1 {
            assert!(
                language_result.top_channels[i].message_count
                    >= language_result.top_channels[i + 1].message_count
            );
        }
    }
}

#[test]
fn test_integration_top_languages_by_viewer_range_only() {
    let test_data = r#"{"channelName": "small1", "language": "en", "viewerCount": 50}
{"channelName": "small2", "language": "es", "viewerCount": 80}
{"channelName": "medium1", "language": "en", "viewerCount": 200}
{"channelName": "large1", "language": "fr", "viewerCount": 800}
{"channelName": "xlarge1", "language": "en", "viewerCount": 1500}"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(test_data.as_bytes())
        .expect("Failed to write test data");

    let file_path = temp_file.path().to_str().unwrap();

    let result = top_languages_by_viewer_range(file_path, 2);
    assert!(
        result.is_ok(),
        "top_languages_by_viewer_range should succeed"
    );

    let results = result.unwrap();
    assert!(!results.is_empty(), "Should have viewer range results");

    let ranges: Vec<&str> = results.iter().map(|r| r.viewer_range.as_str()).collect();
    assert!(ranges.contains(&"0-100"));
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
fn test_integration_with_multiple_threads() {
    let test_data = r#"{"channelName": "channel1", "language": "en", "viewerCount": 100}
{"channelName": "channel2", "language": "en", "viewerCount": 200}
{"channelName": "channel3", "language": "es", "viewerCount": 300}
{"channelName": "channel4", "language": "fr", "viewerCount": 400}
{"channelName": "channel1", "language": "en", "viewerCount": 150}"#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file
        .write_all(test_data.as_bytes())
        .expect("Failed to write test data");

    let file_path = temp_file.path().to_str().unwrap();

    let result_1_thread = analyze_both(file_path, 1);
    let result_2_threads = analyze_both(file_path, 2);
    let result_4_threads = analyze_both(file_path, 4);

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
}
