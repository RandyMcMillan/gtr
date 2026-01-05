use super::error::{GtrError, GitError, ConfigError};
use std::path::PathBuf;

#[test]
fn test_git_error_not_git_repo() {
    let path = PathBuf::from("/nonexistent/path");
    let error = GtrError::not_git_repo(&path);
    assert_eq!(error.to_string(), format!("{} is not a git repository", path.display()));
}

#[test]
fn test_git_error_command_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::Other, "command failed");
    let error = GtrError::command_failed(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "command failed")));
    assert_eq!(error.to_string(), format!("Error running git command, {:#?}", original_error));
}

#[test]
fn test_git_error_ignore_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
    let error = GtrError::ignore_failed(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied")));
    assert_eq!(error.to_string(), format!("Error persisting git ignore, {:#?}", original_error));
}

#[test]
fn test_git_error_pack_read_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken pipe");
    let error = GtrError::pack_read_failed(Box::new(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken pipe")));
    assert_eq!(error.to_string(), format!("Error requesting pack file: {:?}", original_error));
}

#[test]
fn test_git_error_pack_write_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::Interrupted, "interrupted");
    let error = GtrError::pack_write_failed(Box::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "interrupted")));
    assert_eq!(error.to_string(), format!("Error reading pack file content: {:?}", original_error));
}

#[test]
fn test_config_error_save_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "read-only filesystem");
    let error = GtrError::save_failed(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "read-only filesystem")));
    assert_eq!(error.to_string(), format!("Cant save configuration to file {:?}", original_error));
}

#[test]
fn test_config_error_read_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error = GtrError::read_failed(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found")));
    assert_eq!(error.to_string(), format!("Cant read configuration from file {:?}", original_error));
}

#[test]
fn test_config_error_dir_creation_failed() {
    let original_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no write access");
    let error = GtrError::dir_creation_failed(Box::new(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no write access")));
    assert_eq!(error.to_string(), format!("Cant create  gtr directory {:?}", original_error));
}
