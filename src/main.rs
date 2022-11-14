use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

/// Chronological notes
#[derive(Parser, Debug)]
#[command(name = "rittle")]
#[command(about = "Chronological notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, default_value = "rittle")]
    project: String,

    #[arg(long, global = true)]
    prefix: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// New note
    New,

    /// Save notes
    Save,
}

fn main() {
    let args = Cli::parse();

    let rittle_home = format!("{}/.rittle", env::var("HOME").unwrap());
    let project_file_path = format!("{}/{}.md", rittle_home, args.project);

    match args.command {
        Commands::New => {
            let now: DateTime<Utc> = Utc::now();
            let iso_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            let file_name = match args.prefix.as_deref() {
                Some(s) => format!("{}-{}-{}.md", s, args.project, iso_date),
                None => format!("{}-{}.md", args.project, iso_date),
            };

            let mut file = File::create(Path::new(&file_name)).unwrap();

            let content = format!("# {}\n\n## New note\n\n", iso_date);
            file.write_all(content.as_bytes()).unwrap();

            println!("{}", file_name);
        }

        Commands::Save => {
            let expr = match args.prefix.as_deref() {
                Some(s) => format!("^{}-{}.*md$", s, args.project),
                None => format!("^{}.*md$", args.project),
            };
            let re = Regex::new(expr.as_str()).unwrap();

            let mut entries: Vec<DirEntry> = Vec::new();
            for entry_res in WalkDir::new("./") {
                let entry = entry_res.unwrap();
                if re.is_match(entry.file_name().to_str().unwrap()) {
                    entries.push(entry);
                }
            }

            if entries.len() < 1 {
                println!("Didn't find any notes.");
                std::process::exit(0);
            }

            entries.sort_by(|a, b| a.file_name().cmp(b.file_name()));

            for entry in entries {
                if Path::new(&project_file_path).exists() {
                    let project_file_bytes = std::fs::read(&project_file_path).unwrap();
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(entry.path())
                        .unwrap();

                    file.write(String::from("\n\n").as_bytes()).unwrap();
                    file.write(&project_file_bytes).unwrap();

                    std::fs::remove_file(&project_file_path).unwrap();
                }

                let file_bytes = std::fs::read(entry.path()).unwrap();
                let mut project_file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&project_file_path)
                    .unwrap();

                project_file.write(&file_bytes).unwrap();

                std::fs::remove_file(entry.path()).unwrap();
            }
        }
    }
}
