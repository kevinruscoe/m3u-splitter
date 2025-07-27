// m3usplitter.rs, converted from Python version by lakeconstance78@wolke7.net
//
// This program:
//              1. Splits M3U file in files by group
//
// Requirements: Rust with regex and clap crates
//
// Command line: cargo run -- -i input.m3u -o output_directory
//
// TODO:

use clap::Parser;
use std::fs;
use std::path::Path;

// Re-export library functions
pub use m3u_splitter::*;

#[derive(Parser)]
#[command(author, version, about = "Split M3U playlist files by group", long_about = None)]
struct Args {
    /// Input M3U file to split
    #[arg(short, long)]
    input: String,
    
    /// Output directory for split files (defaults to /output)
    #[arg(short, long, default_value = "/output")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Validate input file exists
    if !Path::new(&args.input).exists() {
        eprintln!("Error: Input file '{}' does not exist", args.input);
        std::process::exit(1);
    }
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output)?;
    
    // Read input file
    println!("Reading input file: {}", args.input);
    let content = fs::read_to_string(&args.input)
        .map_err(|e| format!("Failed to read input file '{}': {}", args.input, e))?;
    
    // Display processing information
    println!("----------------------------------------------");
    println!("Splitting {} -> {}", args.input, args.output);
    println!("----------------------------------------------");
    
    // Process M3U content with progress callback
    let stats = process_m3u_content_with_callback(
        &content, 
        Path::new(&args.output),
        |channel_name, group_name, current, total| {
            let percentage = (current as f64 / total as f64 * 100.0) as u32;
            let group_display = if group_name.trim().is_empty() { 
                "ungrouped" 
            } else { 
                group_name 
            };
            
            println!(
                "[{:3}%] ({:4}/{:4}) {} -> {} group", 
                percentage, 
                current, 
                total, 
                channel_name.trim(),
                group_display
            );
        }
    ).map_err(|e| format!("Failed to process M3U content: {e}"))?;
    
    // Display final statistics
    println!("----------------------------------------------");
    println!("✅ Splitting complete!");
    println!("📊 Total channels processed: {}", stats.total_channels);
    println!("📁 Groups created: {}", stats.groups_created.len());
    
    // Show group breakdown
    if !stats.groups_created.is_empty() {
        println!("\n📋 Group breakdown:");
        let mut groups: Vec<_> = stats.groups_created.iter().collect();
        groups.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
        
        for (group, count) in groups {
            let group_display = if group.trim().is_empty() { 
                "ungrouped" 
            } else { 
                group 
            };
            println!("   • {}: {} channels", group_display, count);
        }
    }
    
    Ok(())
}
