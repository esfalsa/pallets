use anyhow::Result;
use clap::Parser;
use pallets::{DumpType, Manager};

/// Download and manage NationStates daily data dumps
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// A nation name or email address to identify you to NationStates
    #[clap(short, long)]
    user: String,

    /// The type of daily data dump to download
    #[clap(short = 't', long = "type")]
    dump_type: DumpType,

    /// The date of daily data dump to download
    #[clap(short, long)]
    date: String,

    /// The format of the date of daily data dump to download
    #[clap(short = 'f', long, default_value = "%Y-%m-%d")]
    date_format: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let date = chrono::NaiveDate::parse_from_str(&args.date, &args.date_format)?;

    let user_agent = format!(
        "{}/{} (by:Esfalsa, usedBy:{})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        args.user
    );

    let manager = Manager::new(&user_agent)?;

    if manager.has_dump(&args.dump_type, date) {
        eprintln!("Dump already exists");
    } else {
        manager.download_dump(&args.dump_type, date)?;
    }

    Ok(())
}
