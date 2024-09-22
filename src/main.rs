mod communication;
mod netflow;
mod pcap;

use std::io;
use std::sync::Arc;

use netflow::NetflowListener;
use pcap::PcapListener;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut pcap_listener = PcapListener;

    // Used to collect all communications from all sources
    let communications = Arc::new(RwLock::new(communication::Communications::default()));

    // Listen for communications from netflow
    let rwlock = communications.clone();
    tokio::spawn(async move {
        let mut netflow_listener = NetflowListener::new("0.0.0.0:9995").await;
        netflow_listener.listen(rwlock).await
    });

    // Listen for communications from pcap
    let rwlock = communications.clone();
    tokio::spawn(async move { pcap_listener.listen(rwlock).await });

    loop {
        for (ip_addr, communication) in communications.read().await.communications.iter() {
            println!(
                "IP Address: {ip_addr} Connections: {:?}",
                communication.connections
            );
        }
        sleep(Duration::from_millis(100)).await;
    }
}
