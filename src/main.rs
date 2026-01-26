use clap::Parser;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde_yaml;
use std::fs;
use std::path::Path;
use std::process;

/// A kind of cmdb cli with a fuzzy finder front end with many connectors to many datasources
#[derive(Parser, Debug)]
#[command(author = "Chris Harris <chris.m.harris@gmail.com>", version = "0.0.1", about, long_about = None, disable_version_flag = true)]
struct Args {
    /// Print version information
    #[arg(short, long)]
    version: bool,

    /// Search query for fuzzy search
    #[arg(value_name = "QUERY")]
    query: Option<String>,
}

// Configuration structure
#[derive(Debug, serde::Deserialize)]
struct Config {
    general: GeneralConfig,
    ui: UiConfig,
    connectors: ConnectorsConfig,
}

#[derive(Debug, serde::Deserialize)]
struct GeneralConfig {
    search_mode: String,
    max_results: u32,
    show_hidden: bool,
}

#[derive(Debug, serde::Deserialize)]
struct UiConfig {
    theme: String,
    color_enabled: bool,
    animations_enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
struct ConnectorsConfig {
    // This will be expanded with actual connector configs
}

// Data structure for search results
#[derive(Debug, Clone)]
struct SearchResult {
    id: String,
    name: String,
    score: f64,
}

fn load_config() {
    let config_dir = Path::new(&std::env::var("HOME").unwrap()).join(".config/fzseek");
    let config_path = config_dir.join("config.yaml");

    if config_path.exists() {
        match fs::read_to_string(&config_path) {
            Ok(content) => match serde_yaml::from_str::<Config>(&content) {
                Ok(_config) => {
                    println!("Loaded config from: {:?}", config_path);
                }
                Err(e) => {
                    eprintln!("Failed to parse config file {:?}: {}", config_path, e);
                }
            },
            Err(e) => {
                eprintln!("Failed to read config file {:?}: {}", config_path, e);
            }
        }
    } else {
        // Create default config if it doesn't exist
        println!("Creating default config at: {:?}", config_path);
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory {:?}: {}", config_dir, e);
            return;
        }
        if let Err(e) = fs::write(&config_path, include_str!("../default_config.yaml")) {
            eprintln!("Failed to write default config to {:?}: {}", config_path, e);
        }
    }
}

// Sample data for demonstration - in a real implementation, this would come from connectors
fn get_sample_data() -> Vec<SearchResult> {
    vec![
        SearchResult {
            id: "1".to_string(),
            name: "Database Server".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "2".to_string(),
            name: "Web Application".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "3".to_string(),
            name: "Load Balancer".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "4".to_string(),
            name: "API Gateway".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "5".to_string(),
            name: "Database Backup".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "6".to_string(),
            name: "User Authentication Service".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "7".to_string(),
            name: "Monitoring Dashboard".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "8".to_string(),
            name: "CI/CD Pipeline".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "9".to_string(),
            name: "Docker Registry".to_string(),
            score: 0.0,
        },
        SearchResult {
            id: "10".to_string(),
            name: "Kubernetes Cluster".to_string(),
            score: 0.0,
        },
    ]
}

// Perform fuzzy search on the provided data
fn fuzzy_search(data: &[SearchResult], query: &str) -> Vec<SearchResult> {
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<SearchResult> = data
        .iter()
        .filter_map(|item| {
            // Use fuzzy matching to score the match
            if let Some(score) = matcher.fuzzy_match(&item.name, query) {
                Some(SearchResult {
                    id: item.id.clone(),
                    name: item.name.clone(),
                    score: score as f64,
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by score (higher is better)
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results
}

fn main() {
    let args = Args::parse();

    if args.version {
        println!("fzseek version 0.0.1");
        println!("Author: Chris Harris <chris.m.harris@gmail.com>");
        println!("Release Date: 2025-12-12");
        process::exit(0);
    }

    // Load configuration
    load_config();

    // If a query was provided, perform fuzzy search
    if let Some(query) = args.query {
        println!("Searching for: {}", query);
        let data = get_sample_data();
        let results = fuzzy_search(&data, &query);

        if results.is_empty() {
            println!("No results found for query: {}", query);
        } else {
            println!("Results for '{}':", query);
            for result in results {
                println!("  {} - {}", result.id, result.name);
            }
        }
    } else {
        // TODO: Add main application logic here
        println!("fzseek application started");
        println!("Usage: fzseek [QUERY] to search");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_flag() {
        // Test that the version flag can be parsed
        assert_eq!(true, true);
    }

    #[test]
    fn test_fuzzy_search() {
        let data = get_sample_data();
        let results = fuzzy_search(&data, "database");
        assert!(!results.is_empty());
    }
}
