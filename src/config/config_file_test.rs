use tempfile::tempdir;
use tokio::fs::{self, read_to_string};

use super::config_file::{
    AddressPort, CONFIG_DIR, CONFIG_FILE, Config, DEFAULT_CONFIG, Torrent, Transport,
    read_or_create,
};

#[tokio::test]
async fn test_config_save() {
    let tmp_dir = tempdir().unwrap();
    let root_dir = tmp_dir.path().to_path_buf();

    let config = Config {
        branches: vec!["main".to_string(), "develop".to_string()],
        transport: Transport {
            torrent: Some(Torrent {
                router: AddressPort {
                    addr: "127.0.0.1".to_string(),
                    port: 6881,
                },
                bind: AddressPort {
                    addr: "0.0.0.0".to_string(),
                    port: 6882,
                },
            }),
        },
    };

    config.save(&root_dir).await.unwrap();

    let config_dir = root_dir.join(CONFIG_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);

    assert!(config_file_path.exists());

    let saved_content = read_to_string(&config_file_path).await.unwrap();
    let expected_content = toml::to_string(&config).unwrap();
    assert_eq!(saved_content, expected_content);

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_read_or_create_not_found() {
    let tmp_dir = tempdir().unwrap();
    let root_dir = tmp_dir.path().to_path_buf();

    let config = read_or_create(&root_dir).await.unwrap();

    assert_eq!(config.branches, DEFAULT_CONFIG.branches);
    assert!(config.transport.torrent.is_none());

    let config_dir = root_dir.join(CONFIG_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);
    assert!(config_file_path.exists());

    let saved_content = read_to_string(&config_file_path).await.unwrap();
    let expected_content = toml::to_string(&DEFAULT_CONFIG).unwrap();
    assert_eq!(saved_content, expected_content);

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_read_or_create_with_existing_valid_config() {
    let tmp_dir = tempdir().unwrap();
    let root_dir = tmp_dir.path().to_path_buf();
    let config_dir = root_dir.join(CONFIG_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);

    fs::create_dir_all(&config_dir).await.unwrap();

    let custom_config = Config {
        branches: vec!["feature/test".to_string()],
        transport: Transport {
            torrent: Some(Torrent {
                router: AddressPort {
                    addr: "192.168.1.1".to_string(),
                    port: 1234,
                },
                bind: AddressPort {
                    addr: "10.0.0.1".to_string(),
                    port: 5678,
                },
            }),
        },
    };
    let custom_config_content = toml::to_string(&custom_config).unwrap();
    tokio::fs::write(&config_file_path, custom_config_content.as_bytes())
        .await
        .unwrap();

    let config = read_or_create(&root_dir).await.unwrap();

    assert_eq!(config.branches, custom_config.branches);
    assert_eq!(
        config.transport.torrent.as_ref().unwrap().router.addr,
        custom_config
            .transport
            .torrent
            .as_ref()
            .unwrap()
            .router
            .addr
    );
    assert_eq!(
        config.transport.torrent.as_ref().unwrap().router.port,
        custom_config
            .transport
            .torrent
            .as_ref()
            .unwrap()
            .router
            .port
    );

    tmp_dir.close().unwrap();
}

#[tokio::test]
async fn test_read_or_create_with_invalid_config() {
    let tmp_dir = tempdir().unwrap();
    let root_dir = tmp_dir.path().to_path_buf();
    let config_dir = root_dir.join(CONFIG_DIR);
    let config_file_path = config_dir.join(CONFIG_FILE);

    fs::create_dir_all(&config_dir).await.unwrap();

    // Write invalid TOML content
    let invalid_content = r#"
branches = ["main"
transport = { torrent = { router = { addr = "127.0.0.1", port = 6881 }, bind = { addr = "0.0.0.0", port = 6882 } } }
"#;
    tokio::fs::write(&config_file_path, invalid_content.as_bytes())
        .await
        .unwrap();

    let config = read_or_create(&root_dir).await.unwrap();

    // Expect DEFAULT_CONFIG due to unwrap_or(DEFAULT_CONFIG)
    assert_eq!(config.branches, DEFAULT_CONFIG.branches);
    assert!(config.transport.torrent.is_none());

    tmp_dir.close().unwrap();
}
