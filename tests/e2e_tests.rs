use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;
use tokio::fs;

const BIN_NAME: &str = "gtr";
const SETTINGS_DIR: &str = ".gtr";
const CONFIG_FILE: &str = "config.toml";

async fn create_test_git_repo(path: &Path) {
    tokio::process::Command::new("git")
        .arg("init")
        .current_dir(path)
        .status()
        .await
        .unwrap();
    tokio::fs::write(path.join("initial_file.txt"), "initial content")
        .await
        .unwrap();
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
async fn e2e_init_command() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();
    create_test_git_repo(&repo_path).await;

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("init")
        .arg("-p")
        .arg(&repo_path)
        .assert()
        .success();

    let config_dir = repo_path.join(SETTINGS_DIR);
    let config_file = config_dir.join(CONFIG_FILE);
    assert!(config_file.exists());

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn e2e_setup_command() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();
    create_test_git_repo(&repo_path).await;

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("setup")
        .arg("-p")
        .arg(&repo_path)
        .assert()
        .success();

    let gitignore_path = repo_path.join(".gitignore");
    let gitignore_content = fs::read_to_string(&gitignore_path).await.unwrap();
    assert!(gitignore_content.contains(SETTINGS_DIR));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn e2e_share_command_new_branch() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();
    create_test_git_repo(&repo_path).await;

    // Create a new branch
    tokio::process::Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("dev")
        .current_dir(&repo_path)
        .status()
        .await
        .unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("share")
        .arg("-p")
        .arg(&repo_path)
        .arg("-b")
        .arg("master,dev")
        .assert()
        .success();

    let config_dir = repo_path.join(SETTINGS_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);
    assert!(config_file_path.exists());

    let config_content = fs::read_to_string(&config_file_path).await.unwrap();
    assert!(config_content.contains("master"));
    assert!(config_content.contains("dev"));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn e2e_list_command() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();
    create_test_git_repo(&repo_path).await;

    // Share master and dev branches
    tokio::process::Command::new(BIN_NAME)
        .arg("share")
        .arg("-p")
        .arg(&repo_path)
        .arg("-b")
        .arg("master,dev")
        .status()
        .await
        .unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("list")
        .arg("-p")
        .arg(&repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("master"))
        .stdout(predicate::str::contains("dev"));

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn e2e_remove_command() {
    let tmp_dir = tempdir().unwrap();
    let repo_path = tmp_dir.path().to_path_buf();
    create_test_git_repo(&repo_path).await;

    // Share master and dev branches
    tokio::process::Command::new(BIN_NAME)
        .arg("share")
        .arg("-p")
        .arg(&repo_path)
        .arg("-b")
        .arg("master,dev")
        .status()
        .await
        .unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("remove")
        .arg("-p")
        .arg(&repo_path)
        .arg("-b")
        .arg("dev")
        .assert()
        .success();

    let config_dir = repo_path.join(SETTINGS_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);
    let config_content = fs::read_to_string(&config_file_path).await.unwrap();

    assert!(config_content.contains("master"));
    assert!(!config_content.contains("dev"));

    tmp_dir.close().unwrap();
}
