use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

use anyhow::Result;
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

fn main() -> Result<()> {
    let args = Cli::parse();

    let rittle_home = format!("{}/.rittle", env::var("HOME")?);
    let project_file_path = format!("{}/{}.md", rittle_home, args.project);

    match args.command {
        Commands::New => {
            let now: DateTime<Utc> = Utc::now();
            let iso_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            let file_name = match args.prefix.as_deref() {
                Some(s) => format!("{}-{}-{}.md", s, args.project, iso_date),
                None => format!("{}-{}.md", args.project, iso_date),
            };

            let mut file = File::create(Path::new(&file_name))?;

            let content = format!("# {}\n\n", iso_date);
            file.write_all(content.as_bytes())?;

            println!("{}", file_name);
        }

        Commands::Save => {
            let mut dir_entries: Vec<DirEntry> = Vec::new();
            for result in WalkDir::new("./") {
                if let Ok(dir_entry) = result {
                    let re = Regex::new(format!("^.*{}-.*T.*Z\\.md$", args.project).as_str())?;
                    if re.is_match(dir_entry.file_name().to_str().unwrap()) {
                        dir_entries.push(dir_entry);
                    }
                }
            }

            if dir_entries.len() < 1 {
                println!("Didn't find any notes.");
                std::process::exit(0);
            }

            dir_entries.sort_by(|a, b| a.file_name().cmp(b.file_name()));

            for entry in dir_entries {
                match std::fs::read(&project_file_path) {
                    Ok(bytes) => {
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(entry.path())?;
                        file.write(String::from("\n\n").as_bytes())?;
                        file.write(&bytes)?;
                        std::fs::remove_file(&project_file_path)?;
                        Ok(())
                    }
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => {
                            std::fs::create_dir_all(Path::new(&rittle_home))?;
                            Ok(())
                        }
                        _ => Err(error),
                    },
                }?;

                let file_bytes = std::fs::read(entry.path())?;
                let mut project_file = File::create(Path::new(&project_file_path))?;
                project_file.write(&file_bytes)?;

                std::fs::remove_file(entry.path())?;
            }
        }
    };

    Ok(())
}
