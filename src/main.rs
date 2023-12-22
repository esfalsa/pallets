use anyhow::Result;
use clap::Parser;
use pallets::{Downloader, DumpType};

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

    let user_agent = format!(
        "{}/{} (by:Esfalsa, usedBy:{})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        args.user
    );

    let downloader = Downloader::new(&user_agent);

    let date = chrono::NaiveDate::parse_from_str(&args.date, &args.date_format)?;

    if let Some(project_dirs) = directories::ProjectDirs::from("", "esfalsa", "pallets") {
        let dump_dir = project_dirs.data_dir().join("dumps");
        std::fs::create_dir_all(&dump_dir)?;
        let dump_path = dump_dir.join(format!("{}-regions.xml.gz", date));
        downloader.download_dump(&args.dump_type, date, dump_path)?;
    } else {
        eprintln!("Could not find valid home directory path");
    }

    Ok(())
}
