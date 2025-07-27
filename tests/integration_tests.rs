use std::fs;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_cli_integration() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path();
    
    // Create a temporary input file
    let input_file = NamedTempFile::new().unwrap();
    let input_content = r#"#EXTM3U
#EXTINF:-1 group-title="Sports",ESPN
https://example.com/espn.m3u8
#EXTINF:-1 group-title="News",CNN
https://example.com/cnn.m3u8
"#;
    fs::write(input_file.path(), input_content).unwrap();
    
    // Run the CLI command
    let output = Command::new("cargo")
        .args(&[
            "run", "--",
            "-i", input_file.path().to_str().unwrap(),
            "-o", output_path.to_str().unwrap()
        ])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "CLI command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check that files were created
    assert!(output_path.join("Sports.m3u").exists());
    assert!(output_path.join("News.m3u").exists());
}

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Split M3U playlist files by group"));
    assert!(stdout.contains("--input"));
    assert!(stdout.contains("--output"));
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("m3u-splitter"));
}
