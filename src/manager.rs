use crate::{symlink, Dump, DumpOrder, DumpType};

use anyhow::{anyhow, Result};
use chrono::{Datelike, NaiveDate};
use directories::ProjectDirs;
use regex::Regex;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

// pub struct DumpListConfig {
//     order: Option<DumpOrder>,
//     kind: Option<DumpType>,
//     date: Option<(NaiveDate, NaiveDate)>,
// }

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

    pub fn get_dump_path(&self, dump_type: &DumpType, date: NaiveDate) -> PathBuf {
        self.directory
            .join(format!("{}-{}-xml.gz", date, &dump_type))
    }

    pub fn download_dump(&self, user_agent: &str, dump: &DumpType, date: NaiveDate) -> Result<()> {
        let url = format!(
            "https://www.nationstates.net/archive/{dump}/{y}-{m:0>2}-{d:0>2}-{dump}-xml.gz",
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

    pub fn delete_dump(&self, dump_type: &DumpType, date: NaiveDate) -> Result<()> {
        let dump_path = self.get_dump_path(dump_type, date);
        std::fs::remove_file(dump_path)?;
        Ok(())
    }

    pub fn has_dump(&self, dump_type: &DumpType, date: NaiveDate) -> bool {
        self.get_dump_path(dump_type, date).exists()
    }

    pub fn list_dumps(&self) -> Result<impl Iterator<Item = Dump>> {
        let regex = Regex::new(r"^(?<date>\d{4}-\d{2}-\d{2})-(?<type>nations|regions)-xml\.gz$")?;

        let file_names = self.directory.read_dir()?.filter_map(move |entry| {
            let Ok(entry) = entry else {
                return None;
            };

            let file_name = entry.file_name();
            let captures = regex.captures(file_name.to_str()?)?;

            let Ok(date) = NaiveDate::parse_from_str(&captures["date"], "%Y-%m-%d") else {
                return None;
            };

            let dump_type = match &captures["type"] {
                "nations" => DumpType::Nations,
                "regions" => DumpType::Regions,
                _ => return None,
            };

            Some(Dump { dump_type, date })
        });

        Ok(file_names)
    }

    pub fn list_dumps_by_config(
        &self,
        order: DumpOrder,
        kind: Option<DumpType>,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) -> Result<impl Iterator<Item = Dump>> {
        let mut res: Vec<Dump> = self.list_dumps()?.collect();

        match order {
            DumpOrder::Ascending => res.sort(),
            DumpOrder::Descending => res.sort_by(|a, b| b.cmp(a)),
        }

        let res = res.into_iter().filter(move |dump| {
            if let Some(ref kind) = kind {
                if &dump.dump_type != kind {
                    return false;
                }
            }

            if let Some(date) = start {
                if dump.date < date {
                    return false;
                }
            }

            if let Some(date) = end {
                if dump.date > date {
                    return false;
                }
            }

            true
        });

        Ok(res)
    }

    pub fn symlink_dumps<P: AsRef<Path>>(&self, dst: P) -> Result<()> {
        symlink::symlink_dir(self.get_directory(), dst)?;
        Ok(())
    }
}
