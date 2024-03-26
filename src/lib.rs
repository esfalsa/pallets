use std::fmt::Display;

mod manager;
mod symlink;

pub use manager::Manager;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DumpType {
    Regions,
    Nations,
}

impl Display for DumpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DumpType::Regions => write!(f, "regions"),
            DumpType::Nations => write!(f, "nations"),
        }
    }
}

#[derive(Debug)]
pub struct Dump {
    pub dump_type: DumpType,
    pub date: chrono::NaiveDate,
}
