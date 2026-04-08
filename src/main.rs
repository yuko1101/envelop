use anyhow::{Context, Ok};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::wrapper::make_wrapper;

mod env;
mod wrapper;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Make {
        target: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        env: PathBuf,
        #[arg(short, long, default_value = "nushell")]
        format: OutputFormat,
    },
    Wrap {
        target: PathBuf,
        #[arg(short, long)]
        env: PathBuf,
        #[arg(short, long, default_value = "nushell")]
        format: OutputFormat,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum OutputFormat {
    Nushell,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Make {
            target,
            output,
            env,
            format,
        } => {
            let env = serde_json::from_str(
                &std::fs::read_to_string(&env)
                    .with_context(|| format!("Failed to read {}", env.display()))?,
            )
            .with_context(|| format!("Failed to parse {}", env.display()))?;
            make_wrapper(target, output, env, format).context("Failed to create the wrapper")?;
        }
        Command::Wrap {
            target,
            env,
            format,
        } => {
            let destination = PathBuf::from(&target).with_file_name(format!(
                ".{}-wrapped",
                &target.file_name().unwrap().to_string_lossy()
            ));
            std::fs::rename(&target, &destination).with_context(|| {
                format!(
                    "Failed to rename {} to {}",
                    target.display(),
                    destination.display()
                )
            })?;

            let env = serde_json::from_str(
                &std::fs::read_to_string(&env)
                    .with_context(|| format!("Failed to read {}", env.display()))?,
            )
            .with_context(|| format!("Failed to parse {}", env.display()))?;

            make_wrapper(destination, target, env, format)
                .context("Failed to create the wrapper")?;
        }
    }
    Ok(())
}
