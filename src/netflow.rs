use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

use netflow_parser::NetflowParser;

use crate::neo4j::Store;

pub struct NetflowListener {
    sock: UdpSocket,
    parsers: HashMap<String, NetflowParser>,
}

impl NetflowListener {
    pub async fn new(addr: &str) -> Self {
        let sock = UdpSocket::bind(addr)
            .await
            .expect("Failed to bind to address");
        NetflowListener {
            sock,
            parsers: HashMap::new(),
        }
    }

    pub async fn listen(&mut self, store: Arc<RwLock<Store>>) -> io::Result<()> {
        loop {
            let mut buf = [0; 65535];

            let (len, addr) = self.sock.recv_from(&mut buf).await?;

            let data = buf[..len].to_vec();
            let data = data.as_slice();

            let flowsets = match self.parsers.get_mut(&addr.to_string()) {
                Some(parser) => parser.parse_bytes_as_netflow_common_flowsets(data),
                None => {
                    let mut new_parser = NetflowParser::default();
                    let result = new_parser.parse_bytes_as_netflow_common_flowsets(data);
                    self.parsers.insert(addr.to_string(), new_parser);
                    result
                }
            };

            store.write().await.netflowsets.extend(flowsets);
        }
    }
}
