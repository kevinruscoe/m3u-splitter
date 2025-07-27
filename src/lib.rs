use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Default filename for channels without a group
const DEFAULT_EMPTY_GROUP_FILENAME: &str = "ungrouped_channels";

/// M3U file header
const M3U_HEADER: &str = "#EXTM3U";

/// Statistics about the M3U processing
#[derive(Debug, Clone)]
pub struct ProcessingStats {
    pub total_channels: usize,
    pub processed_channels: usize,
    pub groups_created: std::collections::HashMap<String, usize>,
}

impl ProcessingStats {
    pub fn new() -> Self {
        Self {
            total_channels: 0,
            processed_channels: 0,
            groups_created: std::collections::HashMap::new(),
        }
    }
}

/// Custom error type for M3U processing
#[derive(Debug)]
pub enum M3uError {
    IoError(std::io::Error),
    RegexError(regex::Error),
    InvalidFormat(String),
}

impl std::fmt::Display for M3uError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            M3uError::IoError(e) => write!(f, "I/O error: {e}"),
            M3uError::RegexError(e) => write!(f, "Regex error: {e}"),
            M3uError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
        }
    }
}

impl std::error::Error for M3uError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            M3uError::IoError(e) => Some(e),
            M3uError::RegexError(e) => Some(e),
            M3uError::InvalidFormat(_) => None,
        }
    }
}

impl From<std::io::Error> for M3uError {
    fn from(error: std::io::Error) -> Self {
        M3uError::IoError(error)
    }
}

impl From<regex::Error> for M3uError {
    fn from(error: regex::Error) -> Self {
        M3uError::RegexError(error)
    }
}

/// Get the compiled regex pattern for M3U entries
fn get_m3u_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        // Pattern for channel information
        // group(1) GROUP-TITLE
        // group(2) CHANNEL-NAME  
        // group(3) CHANNEL-LINK
        Regex::new(r#"#EXTINF:.*group-title="(.*?)".*,(.*)\nhttps?://(.*)\n"#)
            .expect("Failed to compile regex pattern")
    })
}

/// Convert string to title case (first letter of each word capitalized)
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = String::with_capacity(word.len());
                    result.extend(first.to_uppercase());
                    result.push_str(&chars.as_str().to_lowercase());
                    result
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Sanitize filename by replacing invalid characters with underscores
pub fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '"' | '*' | '?' | '<' | '>' | '|' | '&' => '_',
            _ => c,
        })
        .collect()
}

/// Generate a safe filename for a group
fn generate_group_filename(group_name: &str, output_dir: &Path) -> PathBuf {
    if group_name.trim().is_empty() {
        output_dir.join(format!("{DEFAULT_EMPTY_GROUP_FILENAME}.m3u"))
    } else {
        let title_case_name = to_title_case(group_name);
        let safe_name = sanitize_filename(&title_case_name);
        output_dir.join(format!("{safe_name}.m3u"))
    }
}

/// Check if a file needs the M3U header and add it if necessary
fn ensure_m3u_header(file_path: &Path) -> Result<bool, M3uError> {
    if !file_path.exists() {
        return Ok(true); // New file needs header
    }

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    
    if reader.read_line(&mut first_line)? == 0 {
        // Empty file
        return Ok(true);
    }

    if first_line.trim() != M3U_HEADER {
        println!("{} -> {M3U_HEADER} does not exist -> inserting at line 1", 
                file_path.display());
        
        // Read the rest of the file
        let content = fs::read_to_string(file_path)?;
        
        // Write header + existing content
        let new_content = format!("{M3U_HEADER}\n{content}");
        fs::write(file_path, new_content)?;
    }
    
    Ok(false) // Header already handled
}

/// Write a channel entry to the appropriate file
fn write_channel_entry(entry: &str, file_path: &Path, needs_header: bool) -> Result<(), M3uError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    
    if needs_header {
        writeln!(file, "{M3U_HEADER}")?;
    }
    
    writeln!(file, "{}", entry.trim_end())?;
    
    Ok(())
}

/// Process M3U content and split into separate files by group with progress callback
pub fn process_m3u_content_with_callback<F>(
    content: &str, 
    output_dir: &Path,
    mut progress_callback: F
) -> Result<ProcessingStats, M3uError> 
where 
    F: FnMut(&str, &str, usize, usize), // (channel_name, group_name, current, total)
{
    let regex = get_m3u_regex();
    let captures: Vec<_> = regex.captures_iter(content).collect();
    let total = captures.len();
    
    let mut stats = ProcessingStats::new();
    stats.total_channels = total;
    
    for (index, captures) in captures.iter().enumerate() {
        let full_match = captures.get(0)
            .ok_or_else(|| M3uError::InvalidFormat("No full match found".to_string()))?
            .as_str();
            
        let group_name = captures.get(1)
            .ok_or_else(|| M3uError::InvalidFormat("No group name found".to_string()))?
            .as_str();
            
        let channel_name = captures.get(2)
            .ok_or_else(|| M3uError::InvalidFormat("No channel name found".to_string()))?
            .as_str();
        
        // Call progress callback
        progress_callback(channel_name, group_name, index + 1, total);
        
        let file_path = generate_group_filename(group_name, output_dir);
        let needs_header = ensure_m3u_header(&file_path)?;
        
        write_channel_entry(full_match, &file_path, needs_header)?;
        
        // Update statistics
        stats.processed_channels += 1;
        *stats.groups_created.entry(group_name.to_string()).or_insert(0) += 1;
    }
    
    Ok(stats)
}

/// Process M3U content and split into separate files by group
pub fn process_m3u_content(content: &str, output_dir: &Path) -> Result<(), M3uError> {
    process_m3u_content_with_callback(content, output_dir, |_, _, _, _| {})
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("sports"), "Sports");
        assert_eq!(to_title_case("news and entertainment"), "News And Entertainment");
        assert_eq!(to_title_case("MUSIC"), "Music");
        assert_eq!(to_title_case(""), "");
        assert_eq!(to_title_case("multiple   spaces"), "Multiple Spaces");
    }

    #[test]
    fn test_sanitize_filename() {
        let test_cases = vec![
            ("Sports", "Sports"),
            ("News & Entertainment", "News _ Entertainment"),
            ("Movies/TV", "Movies_TV"),
            ("Music: Rock", "Music_ Rock"),
            ("Test*File", "Test_File"),
            ("Valid Name", "Valid Name"),
        ];
        
        for (input, expected) in test_cases {
            let result = sanitize_filename(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_process_m3u_content() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();
        
        let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="Sports",ESPN
https://example.com/espn.m3u8
#EXTINF:-1 group-title="News",CNN
https://example.com/cnn.m3u8
#EXTINF:-1 group-title="Sports",Fox Sports
https://example.com/foxsports.m3u8
"#;
        
        let result = process_m3u_content(m3u_content, output_path);
        assert!(result.is_ok());
        
        // Check that files were created
        let sports_file = output_path.join("Sports.m3u");
        let news_file = output_path.join("News.m3u");
        
        assert!(sports_file.exists());
        assert!(news_file.exists());
        
        // Check content of sports file
        let sports_content = fs::read_to_string(&sports_file).unwrap();
        assert!(sports_content.contains(M3U_HEADER));
        assert!(sports_content.contains("ESPN"));
        assert!(sports_content.contains("Fox Sports"));
        
        // Check content of news file
        let news_content = fs::read_to_string(&news_file).unwrap();
        assert!(news_content.contains(M3U_HEADER));
        assert!(news_content.contains("CNN"));
    }

    #[test]
    fn test_empty_group_handling() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();
        
        let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="",No Group Channel
https://example.com/nogroup.m3u8
"#;
        
        let result = process_m3u_content(m3u_content, output_path);
        assert!(result.is_ok());
        
        // Check that empty group file was created
        let empty_group_file = output_path.join(format!("{}.m3u", DEFAULT_EMPTY_GROUP_FILENAME));
        assert!(empty_group_file.exists());
        
        let content = fs::read_to_string(&empty_group_file).unwrap();
        assert!(content.contains(M3U_HEADER));
        assert!(content.contains("No Group Channel"));
    }

    #[test]
    fn test_extm3u_header_insertion() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();
        
        // Create a file without #EXTM3U header
        let test_file = output_path.join("Test.m3u");
        fs::write(&test_file, "some content without header").unwrap();
        
        let m3u_content = r#"#EXTM3U
#EXTINF:-1 group-title="Test",Test Channel
https://example.com/test.m3u8
"#;
        
        let result = process_m3u_content(m3u_content, output_path);
        assert!(result.is_ok());
        
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.starts_with(M3U_HEADER));
    }
}
