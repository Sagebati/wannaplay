use tokio::prelude::*;
use tokio::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr};
use std::error::Error;
use mac_address::get_mac_address;
use crate::Message;


static IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
static PORT: u16 = 9999;
static ADDR: (IpAddr, u16) = (IP, PORT);

pub struct Client;

impl Client {
    pub async fn send_key(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut stream = TcpStream::connect(ADDR).await?;

        let message = Message {
            mac_addr: get_mac_address()?.unwrap().bytes(),
            key: key.to_string(),
            completed: false,
        };
        let bytes = bincode::serialize(&message)?;
        stream.write_all(&bytes).await?;
        Ok(())
    }

    pub async fn completed(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut stream = TcpStream::connect(ADDR).await?;
        let message = Message {
            mac_addr: get_mac_address()?.unwrap().bytes(),
            key: key.to_string(),
            completed: true,
        };
        stream.write_all(&bincode::serialize(&message)?).await?;
        Ok(())
    }
}


