use netflow_parser::netflow_common::NetflowCommonFlowSet;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

const NEO4J_WRITE_INTERVAL_SECONDS: u64 = 5;

#[derive(Default, Debug)]
pub struct Store {
    pub netflowsets: Vec<NetflowCommonFlowSet>,
}

pub struct Neo4JWriter;

impl Neo4JWriter {
    pub async fn start(&self, store: Arc<RwLock<Store>>) {
        let store = store.read().await;
        // Netflow
        for netflow in &store.netflowsets {
            println!("{:?}", netflow);
        }

        sleep(Duration::from_secs(NEO4J_WRITE_INTERVAL_SECONDS)).await;
    }
}
