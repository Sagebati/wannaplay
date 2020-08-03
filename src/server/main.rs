use tokio::net::*;
use bincode::deserialize;
use tokio::prelude::*;


use log::*;
use wannaplay::{Message, PORT, PATH_DB};
use std::net::Ipv4Addr;
use sled::Db;

#[tokio::main]
async fn main() {
    // Init the Logger
    simple_logger::init_with_level(Level::Info).unwrap();
    // Init the Database
    let tree = sled::open(PATH_DB).expect("Couldn't open the database");
    // Init the server
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let mut listener = TcpListener::bind((addr, PORT))
        .await.expect(&format!("Couldn't bind the  port {}", PORT));
    info!("Server running on localhost:{}", PORT);
    loop {
        let (socket, _) = listener.accept().await.expect("Error accepting the connection");
        process(socket, &tree).await
    }
}

async fn process(mut socket: TcpStream, tree: &Db) {
    info!("Accepted connection from {:?}", socket.peer_addr());
    let mut buff = vec![];

    socket.read_to_end(&mut buff).await.unwrap();
    let message: Message = deserialize(&buff).unwrap();
    info!("Received message: {:?}", message);
    if let Some(old_value) = tree.get(&message.key).unwrap() {
        // Update
        tree.compare_and_swap(&message.key, Some(old_value), Some(buff))
            .unwrap().unwrap();
    } else {
        // Create
        tree
            .insert(&message.key, buff)
            .expect(&format!("Couldn't insert the entry to the database entry {:?}", message));
    }

    debug!("New database: ");
    for res_kv in tree.iter() {
        let (_, v) = res_kv.unwrap();
        debug!("Entry: {:?}", deserialize::<Message>(&v).unwrap());
    }
}