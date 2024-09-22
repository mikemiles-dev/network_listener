use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::net::SocketAddr;
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

    pub async fn listen(&mut self, communication_writer: Arc<RwLock<Communications>>) -> io::Result<()> {
        loop {
            let mut communications = Communications::default();

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

            for flowset in flowsets {
                let ip_addr = match flowset.src_addr {
                    Some(addr) => addr,
                    None => continue,
                };
                let dst_addr = match flowset.dst_addr {
                    Some(addr) => addr,
                    None => continue,
                };
                let dst_port = match flowset.dst_port {
                    Some(port) => port,
                    None => continue,
                };
                let protocol = match flowset.protocol_number {
                    Some(proto) => proto,
                    None => continue,
                };
                let dst_addr = SocketAddr::new(dst_addr, dst_port);
                communications.insert(ip_addr, dst_addr.ip(), dst_port, protocol);
            }
            communication_writer.write().await.merge(communications);
        }
    }
}
