use std::ffi::OsString;
use std::fs;
use std::io;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::communication::Communications;

pub struct PcapListener;

const PCAP_PATH: &str = "./";
const PCAP_EXTENSION: &str = ".pcap";
const PCAP_LISTEN_INTERVAL_MILLIS: u64 = 1000;

impl PcapListener {
    pub async fn listen(
        &mut self,
        communications_writer: Arc<RwLock<Communications>>,
    ) -> io::Result<()> {
        loop {
            let pcaps = Self::get_pcaps_list(PCAP_PATH);
            println!("Found the following pcaps: {pcaps:?}");

            for pcap in pcaps {
                println!("Processing pcap: {pcap:?}");
            }

            sleep(Duration::from_millis(PCAP_LISTEN_INTERVAL_MILLIS)).await;
        }
    }

    // Get list of pcap files
    fn get_pcaps_list(path: &str) -> Vec<OsString> {
        let paths = match fs::read_dir(path) {
            Ok(paths) => paths,
            Err(_) => return Vec::new(),
        };

        let mut pcaps = Vec::new();

        for path in paths.flatten() {
            if let Ok(file_name) = path.file_name().into_string() {
                if file_name.ends_with(PCAP_EXTENSION) {
                    pcaps.push(path.path().into());
                }
            }
        }

        pcaps
    }
}
