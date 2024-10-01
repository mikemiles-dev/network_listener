mod neo4j;
mod netflow;
mod pcap;

use std::io;
use std::sync::Arc;

use neo4j::{Neo4JWriter, Store as Neo4JStore};
use netflow::NetflowListener;
use pcap::PcapListener;
use tokio::sync::RwLock;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Settings {
    /// The user to connect to the database with
    #[arg(short, long, default_value = "neo4j")]
    user: String,
    /// The URI to connect to the database with
    #[arg(short, long, default_value = "neo4j://localhost")]
    server_uri: String,
    /// The password to connect to the database with
    #[arg(short, long, default_value = "neo4j")]
    password: String,
    /// Create the database on startup
    #[arg(short, long, default_value_t = false)]
    create_db: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let settings = Settings::parse();

    let mut pcap_listener = PcapListener;
    let neo4j_writer = Neo4JWriter::new(&settings).await;

    // Create a Neo4j store
    let store = Arc::new(RwLock::new(Neo4JStore::default()));

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
