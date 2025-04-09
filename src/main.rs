use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;
use std::time::{Duration, SystemTime};

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use clipboard::{ClipboardContext, ClipboardProvider};
use directories::ProjectDirs;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "cacheclip")]
#[command(about = "A clipboard history manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List recent clipboard items
    List {
        /// Number of items to show
        #[arg(short, long, default_value_t = 10)]
        count: usize,
    },
    /// Search clipboard history
    Search {
        /// Text to search for
        query: String,
    },
    /// Restore a clipboard item
    Restore {
        /// Item index to restore
        index: usize,
    },
    /// Clear clipboard history
    Clear,
    /// Start monitoring the clipboard in the background
    Daemon,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClipboardItem {
    content: String,
    timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClipboardHistory {
    items: Vec<ClipboardItem>,
}

impl ClipboardHistory {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, content: String) {
        // Don't add empty content
        if content.trim().is_empty() {
            return;
        }

        // Don't add duplicate content (check if last item is the same)
        if let Some(last_item) = self.items.first() {
            if last_item.content == content {
                return;
            }
        }

        let item = ClipboardItem {
            content,
            timestamp: Local::now(),
        };
        
        // Insert at the beginning for reverse chronological order
        self.items.insert(0, item);
        
        // Limit history to 100 items
        if self.items.len() > 100 {
            self.items.truncate(100);
        }
    }

    fn get_item(&self, index: usize) -> Option<&ClipboardItem> {
        self.items.get(index)
    }

    fn search(&self, query: &str) -> Vec<(usize, &ClipboardItem)> {
        let matcher = SkimMatcherV2::default();
        let mut matches = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if let Some(_score) = matcher.fuzzy_match(&item.content, query) {
                matches.push((i, item));
            }
        }

        matches
    }

    fn clear(&mut self) {
        self.items.clear();
    }
}

fn get_history_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "cacheclip", "cacheclip")
        .expect("Could not find home directory");
    
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir).expect("Could not create data directory");
    
    data_dir.join("history.json")
}

fn load_history() -> ClipboardHistory {
    let path = get_history_path();
    
    if path.exists() {
        let mut file = File::open(&path).expect("Could not open history file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read history file");
        
        match serde_json::from_str(&contents) {
            Ok(history) => history,
            Err(_) => {
                eprintln!("Could not parse history file. Starting with a new history.");
                ClipboardHistory::new()
            }
        }
    } else {
        ClipboardHistory::new()
    }
}

fn save_history(history: &ClipboardHistory) {
    let path = get_history_path();
    let content = serde_json::to_string_pretty(history).expect("Could not serialize history");
    
    let mut file = File::create(&path).expect("Could not create history file");
    file.write_all(content.as_bytes()).expect("Could not write history");
}

fn format_item(index: usize, item: &ClipboardItem) -> String {
    let timestamp = item.timestamp.format("%Y-%m-%d %H:%M:%S");
    let content = if item.content.len() > 60 {
        format!("{}...", &item.content[..57])
    } else {
        item.content.clone()
    };
    
    format!("[{}] {} | {}", index, timestamp, content.replace('\n', " "))
}

fn run_daemon() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Could not initialize clipboard");
    let mut last_content = String::new();
    let mut history = load_history();
    
    println!("CacheClip daemon started. Press Ctrl+C to stop.");
    
    loop {
        if let Ok(content) = ctx.get_contents() {
            if !content.is_empty() && content != last_content {
                history.add_item(content.clone());
                save_history(&history);
                last_content = content;
            }
        }
        
        // Sleep to reduce CPU usage
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::List { count }) => {
            let history = load_history();
            let display_count = count.min(history.items.len());
            
            if history.items.is_empty() {
                println!("Clipboard history is empty.");
                return;
            }
            
            println!("Recent clipboard items:");
            for i in 0..display_count {
                println!("{}", format_item(i, &history.items[i]));
            }
        },
        Some(Commands::Search { query }) => {
            let history = load_history();
            let results = history.search(&query);
            
            if results.is_empty() {
                println!("No matching items found.");
                return;
            }
            
            println!("Search results for '{}':", query);
            for (i, item) in results {
                println!("{}", format_item(i, item));
            }
        },
        Some(Commands::Restore { index }) => {
            let history = load_history();
            
            if let Some(item) = history.get_item(index) {
                let mut ctx: ClipboardContext = ClipboardProvider::new()
                    .expect("Could not initialize clipboard");
                ctx.set_contents(item.content.clone()).expect("Could not set clipboard contents");
                println!("Restored item [{}] to clipboard", index);
            } else {
                eprintln!("Item with index {} not found", index);
                process::exit(1);
            }
        },
        Some(Commands::Clear) => {
            let mut history = load_history();
            history.clear();
            save_history(&history);
            println!("Clipboard history cleared.");
        },
        Some(Commands::Daemon) => {
            run_daemon();
        },
        None => {
            println!("No command specified. Use --help for more information.");
        }
    }
}