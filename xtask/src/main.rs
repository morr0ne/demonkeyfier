use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;
use xshell::{Shell, cmd};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Extract,
}

const VERSIONS: [&str; 8] = [
    "SLUS-20685", // USA
    "SCCS-40001", // China(?)
    "SCES-50885", // Europe
    "SCES-51102", // France
    "SCES-51104", // Germany
    "SCES-51103", // Italy
    "SCES-51105", // Spain
    "SCPS-15025", // Original Japanese release
];

fn main() -> Result<()> {
    let Cli { commands } = Cli::parse();

    match commands {
        Commands::Extract => {
            // Check if the files are actually there
            for version in VERSIONS {
                // TODO: do some sort of checksum
                if !fs::exists(format!("isos/{version}.iso"))
                    .context("Failed to verify path existance")?
                {
                    bail!("Missing iso file for {version}")
                }
            }

            if fs::exists("extracted")? {
                fs::remove_dir_all("extracted")?;
            }

            fs::create_dir_all("extracted")?;

            VERSIONS.par_iter().try_for_each(|version| {
                eprintln!("Extracting {version}...");
                let sh = Shell::new()?;

                fs::create_dir_all(format!("extracted/{version}"))?;

                cmd!(
                    sh,
                    "bsdtar -xpkvf isos/{version}.iso -C extracted/{version}"
                )
                .ignore_stdout()
                .ignore_stderr()
                .quiet()
                .run()
                .context("Failed to extract iso")?;

                eprintln!("Extracted {version} successfully");

                anyhow::Ok(())
            })?;
        }
    }

    Ok(())
}
