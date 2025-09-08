// Importar desde tu crate
use TP0ProgramacionConcurrente::error::Error;



#[test]
fn unknown_error_creation() {
    let error = Error::UnknownError("test message".to_string());

    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("UnknownError"));
    assert!(debug_output.contains("test message"));
}
#[test]
fn test_error_transformation_creation() {
    let error = Error::TransformationError("test message".to_string());

    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("TransformationError"));
    assert!(debug_output.contains("test message"));
}
#[test]
fn test_parse_error_creation() {
    let error = Error::ParseError("test message".to_string());


    let debug_output = format!("{:?}", error);
    assert!(debug_output.contains("ParseError"));
    assert!(debug_output.contains("test message"));
}

#[test]
fn test_error_display() {
    let error = Error::IOError("file not found".to_string());
    // Si implementas Display trait
    let display_output = format!("{}", error);
    assert!(display_output.contains("file not found"));
}

