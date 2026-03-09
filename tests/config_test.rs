use putain::config::Config;
use tempfile::TempDir;

#[test]
fn test_default_config() {
    let config = Config::default();
    assert!(config.plugin_dirs.is_empty());
    assert!(config.require_confirmation);
}

#[test]
fn test_config_with_custom_plugin_dir() {
    let tmp = TempDir::new().unwrap();
    let config = Config {
        plugin_dirs: vec![tmp.path().to_path_buf()],
        require_confirmation: false,
    };
    assert_eq!(config.plugin_dirs.len(), 1);
    assert!(!config.require_confirmation);
}
