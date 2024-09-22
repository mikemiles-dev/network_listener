use std::ffi::OsString;
use std::fs;
use std::io;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use pcap_file::pcap::PcapReader;

use crate::communication::Communications;

pub struct PcapListener;

const PCAP_PATH: &str = "./pcaps";
const PCAP_EXTENSION: &str = ".pcap";
const PCAP_PARSED_EXTENSION: &str = ".parsed";
const PCAP_LISTEN_INTERVAL_MILLIS: u64 = 1000;

impl PcapListener {
    pub async fn listen(
        &mut self,
        communications_writer: Arc<RwLock<Communications>>,
    ) -> io::Result<()> {
        loop {
            let pcaps = Self::get_pcaps_list(PCAP_PATH);
            println!("Found the following pcaps: {pcaps:?}");

            // Process each pcap file
            for pcap in pcaps {
                println!("Processing pcap: {pcap:?}");
                let pcap_str = match pcap.to_str() {
                    Some(pcap_str) => pcap_str,
                    None => {
                        println!("Error converting {pcap:?} to string");
                        continue;
                    }
                };
                let communications = Self::process_pcap(pcap_str);

                // Merge communications
                communications_writer.write().await.merge(communications);

                // Rename file
                let mut parsed_filename = pcap.clone();
                parsed_filename.push(PCAP_PARSED_EXTENSION);
                match fs::rename(&pcap, parsed_filename) {
                    Ok(_) => println!("Paresed {pcap:?}"),
                    Err(_) => println!("Error renaming {pcap:?}"),
                }
            }

            sleep(Duration::from_millis(PCAP_LISTEN_INTERVAL_MILLIS)).await;
        }
    }

    fn process_pcap(file_name: &str) -> Communications {
        let mut communications = Communications::default();
        let file_in = fs::File::open(file_name).expect("Error opening file");
        let mut pcap_reader = PcapReader::new(file_in).unwrap();

        // Read test.pcap
        while let Some(pkt) = pcap_reader.next_packet() {
            //Check if there is no error
            if let Ok(pkt) = pkt {
                let (_header, body) = pkt.data.split_at(32);

                // Attempt to parse as Netflow
                let netflow = netflow_parser::NetflowParser::default()
                    .parse_bytes_as_netflow_common_flowsets(body);
                communications.merge(netflow.into());
            }
        }

        communications
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
