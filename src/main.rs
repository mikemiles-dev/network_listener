mod neo4j;
mod netflow;
mod pcap;

use std::io;
use std::sync::Arc;

use neo4j::Store;
use netflow::NetflowListener;
use pcap::PcapListener;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut pcap_listener = PcapListener;

    // Create a Neo4j store
    let store = Arc::new(RwLock::new(Store::default()));

    // Listen for communications from netflow
    let rwlock = store.clone();
    tokio::spawn(async move {
        let mut netflow_listener = NetflowListener::new("0.0.0.0:9995").await;
        netflow_listener.listen(rwlock).await
    });

    // Listen for communications from pcap
    let rwlock = store.clone();
    tokio::spawn(async move { pcap_listener.listen(rwlock).await });

    loop {
        let store = store.read().await;
        for netflow in store.netflowsets.iter() {
            println!("{:?}", netflow);
        }
        sleep(Duration::from_millis(100)).await;
    }
}
