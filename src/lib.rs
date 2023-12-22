use anyhow::{anyhow, Result};
use chrono::Datelike;
use std::{fs::File, path::Path};

pub struct Downloader {
    agent: ureq::Agent,
}

impl Downloader {
    pub fn new(user_agent: &str) -> Self {
        Downloader {
            agent: ureq::AgentBuilder::new().user_agent(user_agent).build(),
        }
    }

    pub fn download_dump<P: AsRef<Path>>(
        &self,
        dump: &DumpType,
        date: chrono::NaiveDate,
        path: P,
    ) -> Result<()> {
        let dump_type = match dump {
            DumpType::Regions => "regions",
            DumpType::Nations => "nations",
        };
        let url = format!(
            "https://www.nationstates.net/archive/{dump_type}/{y}-{m}-{d}-regions-xml.gz",
            y = date.year(),
            m = date.month(),
            d = date.day(),
        );
        let response = self.agent.get(&url).call()?;

        match response.header("Content-Type") {
            Some("application/x-gzip") => {
                let mut file = File::create(path)?;
                std::io::copy(&mut response.into_reader(), &mut file)?;
                Ok(())
            }
            Some(content_type) => Err(anyhow!("Unexpected content type: {}", content_type)),
            None => Err(anyhow!("Could not determine content type")),
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DumpType {
    Regions,
    Nations,
}
