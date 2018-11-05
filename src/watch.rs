use error::Result;

use args::Args;

use systemd::id128::Id128;
use systemd::journal::{Journal, JournalFiles, JournalRecord, JournalSeek};

use std::time::SystemTime;

use chrono::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Systemd {
    pub code_file: String,
    pub code_line: usize,
    pub code_func: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkManagerLogLevel {
    Err,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkManagerDomain {
    Platform,
    Rfkill,
    Ether,
    Wifi,
    Bt,
    Mb,
    Dhcp4,
    Dhcp6,
    Ppp,
    WifiScan,
    Ip4,
    Ip6,
    Autoip4,
    Dns,
    Vpn,
    Sharing,
    Supplicant,
    Agents,
    Settings,
    Suspend,
    Core,
    Device,
    Olpc,
    Wimax,
    Infiniband,
    Firewall,
    Adsl,
    Bond,
    Vlan,
    Bridge,
    DbusProps,
    Team,
    Concheck,
    Dcb,
    Dispatch,
    Audit,
    Systemd,
    VpnPlugin,
    Proxy,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkManager {
    pub log_level: NetworkManagerLogLevel,
    pub domain: NetworkManagerDomain,
    pub device: Option<String>,
    pub code_file: String,
    pub code_line: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Identifier {
    Kernel,
    Systemd(Systemd),
    NetworkManager(NetworkManager),
    NetworkManagerDispatcher,
    WpaSupplicant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    message: String,
    time: DateTime<Utc>,
    identifier: Identifier,
}

fn record_to_entry(record: JournalRecord, timestamp: SystemTime) -> Option<Entry> {
    let identifier = match record.get("SYSLOG_IDENTIFIER")? as &str {
        "kernel" => Identifier::Kernel,
        "systemd" => {
            let code_file = record.get("CODE_FILE")?.clone();
            let code_line = if let Ok(code_line) = record.get("CODE_LINE")?.parse::<usize>() {
                code_line
            } else {
                return None;
            };
            let code_func = record.get("CODE_FUNC")?.clone();
            Identifier::Systemd(Systemd {
                code_file,
                code_line,
                code_func,
            })
        }
        "NetworkManager" => {
            let code_file = record.get("CODE_FILE")?.clone();
            let code_line = if let Ok(code_line) = record.get("CODE_LINE")?.parse::<usize>() {
                code_line
            } else {
                return None;
            };
            let log_level = NetworkManagerLogLevel::Info;
            let domain = NetworkManagerDomain::Core;
            let device = None;
            Identifier::NetworkManager(NetworkManager {
                log_level,
                domain,
                device,
                code_file,
                code_line,
            })
        }
        "nm-dispatcher" => Identifier::NetworkManagerDispatcher,
        "wpa_supplicant" => Identifier::WpaSupplicant,
        _ => return None,
    };

    let message = record.get("MESSAGE")?.clone();

    let time = DateTime::<Utc>::from(timestamp);

    Some(Entry {
        message,
        time,
        identifier,
    })
}

pub fn watch(_args: &Args) -> Result<()> {
    let boot_id = Id128::from_boot()?;

    let mut journal = Journal::open(JournalFiles::All, false, false)?;

    journal.match_add("_BOOT_ID", boot_id.to_string().as_str())?;

    journal.seek(JournalSeek::Head)?;

    let mut entries = vec![];

    loop {
        match journal.next_record()? {
            Some(record) => {
                let timestamp = journal.timestamp()?;

                if let Some(entry) = record_to_entry(record, timestamp) {
                    entries.push(entry);
                }
            }
            None => break,
        }
    }

    for entry in &entries[..1000] {
        println!("{:?}", entry);
    }

    Ok(())
}
