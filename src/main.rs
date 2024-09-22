mod communication;
mod netflow;

use std::io;

use netflow::NetflowListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut netflow_listener = NetflowListener::new("0.0.0.0:9995").await;

    let mut communications = communication::Communications::default();

    loop {
        let netflow_communications = netflow_listener.listen().await?;

        communications.merge(netflow_communications);

        for (ip_addr, communication) in communications.communications.iter() {
            println!("IP Address: {ip_addr} Communications: {:?}", communication);
        }
    }
}
