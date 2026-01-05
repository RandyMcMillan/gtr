use super::error::{GtrError, GitError, ConfigError};
use std::path::PathBuf;

#[test]
fn test_gtr_error_new() {
    let error_message = "Test error message".to_string();
    let gtr_error = GtrError::new(error_message.clone());
    assert_eq!(gtr_error.to_string(), error_message);
    assert_eq!(gtr_error.message, error_message);
}

#[test]
fn test_gtr_error_display() {
    let error_message = "Display test message".to_string();
    let gtr_error = GtrError::new(error_message.clone());
    assert_eq!(format!("{}", gtr_error), error_message);
}

#[test]
fn test_git_error_not_git_repo() {
    let path = PathBuf::from("/nonexistent/path");
    let error = GtrError::not_git_repo(&path);
    assert_eq!(error.to_string(), format!("{} is not a git repository", path.display()));
}

#[test]
fn test_git_error_command_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::Other, "command failed");
    let error = GtrError::command_failed(Box::new(original_error));
    // Using `contains` because the debug format of `io::Error` includes more details
    assert!(error.to_string().contains("Error running git command, Custom { kind: Other, error: \"command failed\" }"));
}

#[test]
fn test_git_error_ignore_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
    let error = GtrError::ignore_failed(Box::new(original_error));
    assert!(error.to_string().contains("Error persisting git ignore, Custom { kind: PermissionDenied, error: \"permission denied\" }"));
}

#[test]
fn test_git_error_pack_read_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken pipe");
    let error = GtrError::pack_read_failed(Box::new(original_error));
    assert!(error.to_string().contains("Error requesting pack file: Custom { kind: BrokenPipe, error: \"broken pipe\" }"));
}

#[test]
fn test_git_error_pack_write_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::Interrupted, "interrupted");
    let error = GtrError::pack_write_failed(Box::new(original_error));
    assert!(error.to_string().contains("Error reading pack file content: Custom { kind: Interrupted, error: \"interrupted\" }"));
}

#[test]
fn test_config_error_save_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "read-only filesystem");
    let error = GtrError::save_failed(Box::new(original_error));
    assert!(error.to_string().contains("Cant save configuration to file Custom { kind: PermissionDenied, error: \"read-only filesystem\" }"));
}

#[test]
fn test_config_error_read_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = GtrError::read_failed(Box::new(original_error));
    assert!(error.to_string().contains("Cant read configuration from file Custom { kind: NotFound, error: \"file not found\" }"));
}

#[test]
fn test_config_error_dir_creation_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no write access");
    let error = GtrError::dir_creation_failed(Box::new(original_error));
    assert!(error.to_string().contains("Cant create  gtr directory Custom { kind: PermissionDenied, error: \"no write access\" }"));
}
