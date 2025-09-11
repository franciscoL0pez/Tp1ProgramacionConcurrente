//parser tests
use TP0ProgramacionConcurrente::parser::parse_message;
use TP0ProgramacionConcurrente::parser::parse_json_file;

#[test]
fn test_parse_message() {
    let json_str = r#"
    {
        "channel_name": "test_channel",
        "language": "en",
        "viewer_count": 150
    }
    "#;

    let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    let message = parse_message(&json_value).unwrap();

    assert_eq!(message.channel_name, "test_channel");
    assert_eq!(message.language, "en");
    assert_eq!(message.viewer_count, 150);
}

