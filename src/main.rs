use anyhow::{anyhow, Result};
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

        /// Force download even if the dump already exists
        #[clap(long)]
        force: bool,
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

    /// List all downloaded daily data dumps
    List,

    /// Get the path to the directory where daily data dumps are stored
    Prefix,

    /// Create a symbolic link to the folder containing all data dumps
    Link {
        /// The path to the symbolic link to create
        #[clap(default_value = ".")]
        path: String,
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
        Command::Download { user, dump, force } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            let user_agent = format!(
                "{}/{} (by:Esfalsa, usedBy:{})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                user
            );

            if !force && manager.has_dump(&dump.dump_type, date) {
                return Err(anyhow!("Dump already exists"));
            } else {
                manager.download_dump(&user_agent, &dump.dump_type, date)?;
            }
        }
        Command::Delete { dump } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            if manager.has_dump(&dump.dump_type, date) {
                manager.delete_dump(&dump.dump_type, date)?;
            } else {
                return Err(anyhow!("Dump does not exist"));
            }
        }
        Command::Path { dump } => {
            let date = chrono::NaiveDate::parse_from_str(&dump.date, &dump.date_format)?;

            if manager.has_dump(&dump.dump_type, date) {
                println!("{}", manager.get_dump_path(&dump.dump_type, date).display());
            } else {
                return Err(anyhow!("Dump does not exist"));
            }
        }
        Command::List => {
            for dump in manager.list_dumps()? {
                println!("{} {}", dump.dump_type, dump.date);
            }
        }
        Command::Prefix => {
            println!("{}", manager.get_directory().display());
        }
        Command::Link { path } => {
            let path = std::path::Path::new(&path);

            if !path.exists() {
                manager.symlink_dumps(path)?;
            } else if path.is_dir() {
                let dumps = path.join("dumps");
                if dumps.exists() {
                    return Err(anyhow!("Path {} already exists", dumps.display()));
                } else {
                    manager.symlink_dumps(dumps)?;
                }
            } else {
                return Err(anyhow!("Path {} already exists", path.display()));
            }
        }
    }

    Ok(())
}
