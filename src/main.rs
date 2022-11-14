use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Take notes
#[derive(Parser, Debug)]
#[command(name = "rittle")]
#[command(about = "Take notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, default_value = "rittle")]
    project: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// New note
    New,

    /// Compile notes
    Compile,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::New => {
            let now: DateTime<Utc> = Utc::now();
            let iso_date = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            let file_name = format!("{}-{}.md", args.project.to_owned(), iso_date.to_owned());

            let mut file = File::create(Path::new(&file_name)).unwrap();

            let content = format!("# {}\n\n# New note\n\n", iso_date);
            file.write_all(content.as_bytes()).unwrap();

            println!("{}", file_name);
        }

        Commands::Compile => {
            println!("compile");
        }
    }
}
