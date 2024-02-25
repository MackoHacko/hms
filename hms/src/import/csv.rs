use anyhow::{bail, Context, Ok, Result};
use csv::Reader;
use hms_db::models::NewSnip;
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, serde::Deserialize)]
struct SnipCsvRow {
    alias: String,
    value: String,
}

#[derive(Debug)]
pub struct SnipCsv {
    rows: Vec<SnipCsvRow>,
}

impl<'a> SnipCsv {
    pub fn from_file(fp: PathBuf) -> Result<Self> {
        if !fp.exists() {
            bail!("File not found: {:?}", fp.display());
        }
        let rdr = Reader::from_path(&fp)
            .with_context(|| format!("Failed to read CSV file: {:?}", fp.display()))?;

        let mut rows = Vec::new();
        let mut aliases = HashSet::new();
        let mut duplicates = HashSet::new();

        for result in rdr.into_deserialize::<SnipCsvRow>() {
            let row = result?;
            if !aliases.insert(row.alias.clone()) {
                duplicates.insert(row.alias);
            } else {
                rows.push(row);
            }
        }

        if !duplicates.is_empty() {
            bail!("Csv contains non unique aliases: {:?}", duplicates);
        }

        Ok(Self { rows })
    }

    pub fn to_new_snips(&'a self) -> Vec<NewSnip<'a>> {
        self.rows
            .iter()
            .map(|row| NewSnip {
                alias: &row.alias,
                value: &row.value,
            })
            .collect()
    }
}
