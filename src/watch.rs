use errors::*;

use args::Args;

use systemd::id128::Id128;
use systemd::journal::{Journal, JournalFiles, JournalSeek};

use serde_yaml;

use std::collections::BTreeMap;

lazy_static! {
    static ref RECORD_KEYS: BTreeMap<&'static str, &'static str> = {
        let mut m = BTreeMap::new();
        m.insert("MESSAGE", "M");
        m.insert("NM_LOG_DOMAINS", "ND");
        m.insert("NM_LOG_LEVEL", "NL");
        m.insert("NM_DEVICE", "ND");
        m.insert("SYSLOG_FACILITY", "SF");
        m.insert("SYSLOG_IDENTIFIER", "SI");
        m.insert("CODE_FILE", "CF");
        m.insert("CODE_LINE", "CL");
        m.insert("TIMESTAMP_BOOTTIME", "TB");
        m.insert("TIMESTAMP_MONOTONIC", "TM");
        m.insert("_SOURCE_MONOTONIC_TIMESTAMP", "SM");
        m.insert("_SOURCE_REALTIME_TIMESTAMP", "SR");
        m
    };
}

pub fn watch(_args: &Args) -> Result<()> {
    let boot_id = Id128::from_boot()?;

    let mut journal = Journal::open(JournalFiles::All, false, false)?;

    journal.match_add("_BOOT_ID", boot_id.to_string().as_str())?;

    journal.seek(JournalSeek::Head)?;

    let mut v = vec![];

    loop {
        match journal.next_record()? {
            Some(record) => {
                let mut filtered: BTreeMap<String, String> = BTreeMap::new();

                for (key, value) in record.iter() {
                    if let Some(short) = RECORD_KEYS.get(key as &str) {
                        filtered.insert(short.to_string(), value.to_string());
                    }
                }

                v.push(filtered);
            }
            None => break,
        }
    }

    let s = serde_yaml::to_string(&v).unwrap();
    println!("{}", s);

    Ok(())
}
