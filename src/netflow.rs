use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;

use netflow_parser::NetflowParser;

use crate::communication::Communications;

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

    pub async fn listen(
        &mut self,
        communication_writer: Arc<RwLock<Communications>>,
    ) -> io::Result<()> {
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

            communication_writer.write().await.merge(flowsets.into());
        }
    }
}
