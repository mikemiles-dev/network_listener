use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::net::IpAddr;
use std::time::SystemTime;

use netflow_parser::netflow_common::NetflowCommonFlowSet;

pub type Protocol = u8;

// Implement the Communications struct
#[derive(Default, Debug, Clone)]
pub struct Communications {
    pub communications: HashMap<IpAddr, Communication>,
}

// Implement the insert method for the Communications struct
impl Communications {
    // Implement the insert method for the Communications struct
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
            created_at: SystemTime::now(),
        };
        let connections = communication_record
            .connections
            .entry(dst_ip_addr)
            .or_default();
        connections.remove(&connection);
        connections.insert(connection);
        communication_record.updated_at = SystemTime::now();
    }

    // Implement the merge method for the Communications struct
    pub fn merge(&mut self, other: Communications) {
        // Iterate over the communications in the other Communications struct
        for (ip_addr, new_communication) in other.communications.iter() {
            // Get the communication record for the IP address
            let existing_communication_record = match self.communications.get_mut(ip_addr) {
                Some(record) => {
                    // Update the communication record with the new communication
                    record.updated_at = SystemTime::now();
                    record
                }
                // If the IP address does not exist, create a new communication record
                None => {
                    self.communications
                        .insert(*ip_addr, new_communication.clone());
                    self.communications.get_mut(ip_addr).unwrap()
                }
            };
            // Iterate over the connections in the new communication
            for (dst_ip_addr, new_connections) in new_communication.connections.iter() {
                for new_connection in new_connections.iter() {
                    match existing_communication_record
                        .connections
                        .get_mut(dst_ip_addr)
                    {
                        Some(existing_connections) => {
                            existing_connections.remove(new_connection);
                            let mut new_connection = new_connection.clone();
                            new_connection.updated_at = SystemTime::now();
                            existing_connections.insert(new_connection);
                        }
                        None => {
                            let mut new_connections = HashSet::new();
                            new_connections.insert(new_connection.clone());
                            existing_communication_record
                                .connections
                                .insert(*dst_ip_addr, new_connections);
                        }
                    }
                }
            }
        }
    }
}

// Implement the Debug trait for the Connection struct
#[derive(Debug, Eq, Clone)]
pub struct Connection {
    pub port: u16,
    pub protocol: Protocol,
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.port.hash(state);
        self.protocol.hash(state);
    }
}

// Implement the PartialEq trait for the Connection struct
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port && self.protocol == other.protocol
    }
}

// Implement the new method for the Connection struct
#[derive(Debug, Clone)]
pub struct Communication {
    pub connections: HashMap<IpAddr, HashSet<Connection>>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

// Implement the new method for the Communication struct
impl Communication {
    fn new() -> Self {
        Communication {
            connections: HashMap::new(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

// Implement the From trait for the Communications struct
impl From<Vec<NetflowCommonFlowSet>> for Communications {
    fn from(val: Vec<NetflowCommonFlowSet>) -> Self {
        let mut communications = Communications::default();
        for flowset in val {
            let src_ip_addr = match flowset.src_addr {
                Some(addr) => addr,
                None => continue,
            };
            let dst_ip_addr = match flowset.dst_addr {
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
            communications.insert(src_ip_addr, dst_ip_addr, dst_port, protocol);
        }
        communications
    }
}
