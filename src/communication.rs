use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::net::{IpAddr, SocketAddr};
use std::time::SystemTime;

pub type Protocol = u8;

#[derive(Default, Debug)]
pub struct Communications {
    pub communications: HashMap<IpAddr, Communication>,
}

impl Communications {
    pub fn insert(
        &mut self,
        src_ip_addr: IpAddr,
        dst_ip_addr: IpAddr,
        port: u16,
        protocol: Protocol,
    ) {
        let communication_record = self
            .communications
            .entry(src_ip_addr)
            .or_insert(Communication::new());
        let connection = Connection {
            port,
            protocol,
            updated_at: SystemTime::now(),
        };
        let connections = communication_record
            .connections
            .entry(dst_ip_addr)
            .or_insert(HashSet::new());
        connections.remove(&connection);
        connections.insert(connection);
        communication_record.updated_at = SystemTime::now();
    }

    pub fn merge(&mut self, other: Communications) {
        for (ip_addr, communication) in other.communications.iter() {
            let communication_record = self
                .communications
                .entry(*ip_addr)
                .or_insert(Communication::new());
            for (dst_ip_addr, connections) in communication.connections.iter() {
                for connection in connections.iter() {
                    let connections = communication_record
                        .connections
                        .entry(*dst_ip_addr)
                        .or_insert(HashSet::new()); 
                    connections.remove(connection);
                    connections.insert(connection.clone());
                }
            }
            communication_record.updated_at = SystemTime::now();
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Connection {
    pub port: u16,
    pub protocol: Protocol,
    updated_at: SystemTime,
}

impl Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.port.hash(state);
        self.protocol.hash(state);
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.protocol == other.protocol
    }
}

#[derive(Debug)]
pub struct Communication {
    pub connections: HashMap<IpAddr, HashSet<Connection>>,
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
