use neo4rs::{Error, Graph};
use netflow_parser::netflow_common::NetflowCommonFlowSet;

use core::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::Settings;

const NEO4J_WRITE_INTERVAL_SECONDS: u64 = 5;

#[derive(Default, Debug)]
pub struct Store {
    pub netflowsets: Vec<NetflowCommonFlowSet>,
}

#[derive(Debug)]
pub enum Neo4JError {
    Connection,
    ExistingDatabase(Error),
    Commit(Error),
}

impl fmt::Display for Neo4JError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Neo4JError::Connection => write!(f, "Error connecting to Neo4j"),
            Neo4JError::ExistingDatabase(e) => {
                write!(f, "Error creating database: {:?}", e)
            }
            Neo4JError::Commit(e) => write!(f, "Error committing transaction: {:?}", e),
        }
    }
}

pub struct Neo4JWriter {
    graph: Graph,
}

impl Neo4JWriter {
    pub async fn new(settings: &Settings) -> Neo4JWriter {
        println!("Connecting to Neo4j with uri: {}", &settings.server_uri);
        let graph = Graph::new(&settings.server_uri, &settings.user, &settings.password)
            .await
            .unwrap();
        println!("Connected to Neo4j");
        let neo4j_writer = Neo4JWriter { graph };

        if settings.create_db {
            match neo4j_writer.create_db().await {
                Ok(_) => println!("Database created"),
                Err(e) => println!("Error creating database: {e}"),
            }
        }

        neo4j_writer
    }

    pub async fn start(&self, store: Arc<RwLock<Store>>) {
        let store = store.read().await;
        // Netflow
        for netflow in &store.netflowsets {
            println!("{:?}", netflow);
        }

        sleep(Duration::from_secs(NEO4J_WRITE_INTERVAL_SECONDS)).await;
    }

    async fn create_db(&self) -> Result<(), Neo4JError> {
        //Transactions
        let mut txn = self
            .graph
            .start_txn()
            .await
            .map_err(|_| Neo4JError::Connection)?;
        txn.run_queries(["CREATE database foo"])
            .await
            .map_err(Neo4JError::ExistingDatabase)?;
        txn.commit().await.map_err(Neo4JError::Commit)?; //or txn.rollback().await.unwrap();
        Ok(())
    }
}
