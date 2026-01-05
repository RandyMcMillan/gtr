use super::git_interface::{gtr_setup, is_git, SETTINGS_DIR};
use std::path::Path;
use tempfile::tempdir;
use tokio::fs;

async fn create_test_git_repo(path: &Path) {
    tokio::process::Command::new("git")
        .arg("init")
        .current_dir(path)
        .status()
        .await
        .unwrap();
    // Create an initial commit to have a valid git repo
    tokio::fs::write(path.join("initial_file.txt"), "initial content").await.unwrap();
    tokio::process::Command::new("git")
        .arg("add")
        .arg("initial_file.txt")
        .current_dir(path)
        .status()
        .await
        .unwrap();
    tokio::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .current_dir(path)
        .status()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_gtr_setup_new_repo() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();

    create_test_git_repo(&repo_path).await;

    gtr_setup(&repo_path).await.unwrap();

    let gitignore_path = repo_path.join(".gitignore");
    let gitignore_content = fs::read_to_string(&gitignore_path).await.unwrap();

    assert!(gitignore_content.contains(SETTINGS_DIR));
    assert!(is_git(&repo_path));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_gtr_setup_existing_gitignore() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();

    create_test_git_repo(&repo_path).await;

    // Create a .gitignore file with some existing content
    let gitignore_path = repo_path.join(".gitignore");
    fs::write(&gitignore_path, "/target\n").await.unwrap();

    gtr_setup(&repo_path).await.unwrap();

    let gitignore_content = fs::read_to_string(&gitignore_path).await.unwrap();

    assert!(gitignore_content.contains("/target"));
    assert!(gitignore_content.contains(SETTINGS_DIR));
    assert!(is_git(&repo_path));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_gtr_setup_already_ignored() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();

    create_test_git_repo(&repo_path).await;

    // Create a .gitignore file with SETTINGS_DIR already present
    let gitignore_path = repo_path.join(".gitignore");
    fs::write(&gitignore_path, SETTINGS_DIR).await.unwrap();

    gtr_setup(&repo_path).await.unwrap();

    let gitignore_content = fs::read_to_string(&gitignore_path).await.unwrap();

    // Ensure it's not duplicated
    assert_eq!(gitignore_content.matches(SETTINGS_DIR).count(), 1);
    assert!(is_git(&repo_path));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_gtr_setup_not_git_repo() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();

    // Do not initialize as a git repo
    let result = gtr_setup(&repo_path).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), format!("{} is not a git repository", repo_path.display()));

    tmp_dir.close().unwrap();
}
