use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "navilang")]
#[command(about = "NaviLang compiler and toolchain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse NaviLang file and output AST
    Parse {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Generate diagrams from NaviLang
    Generate {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Validate NaviLang syntax
    Check {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Parse { file, output } => {
            // TODO: Implement parsing
            println!("Parsing file: {:?}", file);
            if let Some(output) = output {
                println!("Output to: {:?}", output);
            }
        }
        Commands::Generate { file, format, output } => {
            // TODO: Implement generation
            println!("Generating {} from {:?}", format, file);
            if let Some(output) = output {
                println!("Output to: {:?}", output);
            }
        }
        Commands::Check { file } => {
            // TODO: Implement validation
            println!("Checking file: {:?}", file);
        }
    }
    
    Ok(())
}
