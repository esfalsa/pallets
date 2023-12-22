use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use pallets::{DumpType, Manager};

/// Download and manage NationStates daily data dumps
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Download a daily data dump
    Download {
        /// A nation name or email address to identify you to NationStates
        #[clap(short, long)]
        user: String,

        #[clap(flatten)]
        dump: DumpArgs,
    },

    /// Delete a daily data dump
    Delete {
        #[clap(flatten)]
        dump: DumpArgs,
    },

    /// Get the path to a daily data dump
    Path {
        #[clap(flatten)]
        dump: DumpArgs,
    },
}

#[derive(Args)]
#[group(multiple = true)]
struct DumpArgs {
    /// The type of daily data dump to get the path of
    #[clap(short = 't', long = "type")]
    dump_type: DumpType,

    /// The date of daily data dump to get the path of
    #[clap(short, long)]
    date: String,

    /// The format of the date of daily data dump to get the path of
    #[clap(short = 'f', long, default_value = "%Y-%m-%d")]
    date_format: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let manager = Manager::new()?;

    match args.command {
        Command::Download { user, dump } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            let user_agent = format!(
                "{}/{} (by:Esfalsa, usedBy:{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                user
            );

            if manager.has_dump(&dump.dump_type, date) {
                eprintln!("Dump already exists");
            } else {
                manager.download_dump(&user_agent, &dump.dump_type, date)?;
            }
        }
        Command::Delete { dump } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            if manager.has_dump(&dump.dump_type, date) {
                manager.delete_dump(&dump.dump_type, date)?;
            } else {
                eprintln!("Dump does not exist");
            }
        }
        Command::Path { dump } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            if manager.has_dump(&dump.dump_type, date) {
                println!("{}", manager.get_dump_path(&dump.dump_type, date).display());
            } else {
                eprintln!("Dump does not exist");
            }
        }
    }

    Ok(())
}
