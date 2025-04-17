use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long, value_name = "FILE")]
    config: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Process {
        #[arg(short, long, default_value_t = 10)]
        count: u32,
    },
    Compute {
        #[arg(short, long, default_value_t = 1000)]
        iterations: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{}", "CLI Tool Started".bright_green().bold());
    match &cli.command {
        Some(Commands::Process { count }) => {
            process_with_progress(*count).await?;
        }
        Some(Commands::Compute { iterations }) => {
            parallel_compute(*iterations)?;
        }
        None => {
            println!("{}", "No command specified. Use --help for usage information.".yellow());
        }
    }
    Ok(())
}

async fn process_with_progress(count: u32) -> Result<()> {
    let pb = ProgressBar::new(count as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")?);
    for i in 0..count {
        pb.set_message(format!("Processing item #{}", i + 1));
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        pb.inc(1);
    }
    pb.finish_with_message("Processing complete!");
    Ok(())
}

fn parallel_compute(iterations: u32) -> Result<()> {
    use rayon::prelude::*;
    println!("{}", "Starting batch computation...".blue());
    let result: u32 = (0..iterations).into_par_iter()
        .map(|i| i * i)
        .sum();
    println!("{} {}", "Computation result:".green(), result);
    Ok(())
}
