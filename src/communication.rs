use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::SystemTime;

pub type Protocol = u8;

#[derive(Default, Debug)]
pub struct Communications {
    pub communications: HashMap<IpAddr, Communication>,
}

impl Communications {
    pub fn insert(&mut self, ip_addr: IpAddr, dst: SocketAddr, protocol: Protocol) {
        let communication_record = self
            .communications
            .entry(ip_addr)
            .or_insert(Communication::new());
        communication_record.connections.insert(dst, protocol);
        communication_record.updated_at = SystemTime::now();
    }

    pub fn merge(&mut self, other: Communications) {
        for (ip_addr, communication) in other.communications {
            let communication_record = self
                .communications
                .entry(ip_addr)
                .or_insert(Communication::new());
            for (dst, protocol) in communication.connections {
                communication_record.connections.insert(dst, protocol);
            }
            communication_record.updated_at = communication.updated_at;
        }
    }
}

#[derive(Debug)]
pub struct Communication {
    pub connections: HashMap<SocketAddr, Protocol>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Communication {
    fn new() -> Self {
        Communication {
            connections: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}
