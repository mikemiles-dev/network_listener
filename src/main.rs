mod neo4j;
mod netflow;
mod pcap;

use std::io;
use std::sync::Arc;

use neo4j::{Neo4JWriter, Store};
use netflow::NetflowListener;
use pcap::PcapListener;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut pcap_listener = PcapListener;
    let neo4j_writer = Neo4JWriter;

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
        let rwlock = store.clone();
        neo4j_writer.start(rwlock).await;
    }
}
