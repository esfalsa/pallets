use anyhow::{anyhow, Result};
use chrono::Datelike;
use directories::ProjectDirs;
use std::{fmt::Display, fs::File, path::PathBuf};

pub struct Manager {
    directory: PathBuf,
}

impl Manager {
    pub fn new() -> Result<Self> {
        if let Some(project_dirs) = ProjectDirs::from("", "esfalsa", "pallets") {
            let directory = project_dirs.data_dir().join("dumps");
            std::fs::create_dir_all(&directory)?;
            Ok(Manager { directory })
        } else {
            Err(anyhow!("Could not find valid home directory path"))
        }
    }

    pub fn get_directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn get_dump_path(&self, dump_type: &DumpType, date: chrono::NaiveDate) -> PathBuf {
        self.directory
            .join(format!("{}-{}.xml.gz", date, &dump_type))
    }

    pub fn download_dump(
        &self,
        user_agent: &str,
        dump: &DumpType,
        date: chrono::NaiveDate,
    ) -> Result<()> {
        let url = format!(
            "https://www.nationstates.net/archive/{dump}/{y}-{m}-{d}-{dump}-xml.gz",
            y = date.year(),
            m = date.month(),
            d = date.day(),
        );
        let response = ureq::get(&url).set("User-Agent", user_agent).call()?;

        match response.header("Content-Type") {
            Some("application/x-gzip") => {
                let mut file = File::create(self.get_dump_path(dump, date))?;
                std::io::copy(&mut response.into_reader(), &mut file)?;
                Ok(())
            }
            Some(content_type) => Err(anyhow!("Unexpected content type: {}", content_type)),
            None => Err(anyhow!("Could not determine content type")),
        }
    }

    pub fn delete_dump(&self, dump_type: &DumpType, date: chrono::NaiveDate) -> Result<()> {
        let dump_path = self.get_dump_path(dump_type, date);
        std::fs::remove_file(dump_path)?;
        Ok(())
    }

    pub fn has_dump(&self, dump_type: &DumpType, date: chrono::NaiveDate) -> bool {
        self.get_dump_path(dump_type, date).exists()
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DumpType {
    Regions,
    Nations,
}

impl Display for &DumpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DumpType::Regions => write!(f, "regions"),
            DumpType::Nations => write!(f, "nations"),
        }
    }
}
