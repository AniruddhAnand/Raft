use std::env;
use std::fs;
use std::collections::HashSet;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
//use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::Duration;
//use log::{info, error};

mod raft_node;
use raft_node::RaftNode;
mod kv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //env_logger::init();

    //TODO: Grab Client Port -> will figure out this once raft can atleast talk to each other
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    let port = match args[1].parse::<u32>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number");
            std::process::exit(1);
        }
    };

    //TODO: Custom Server file
    let server_ports: Vec<u32> = fs::read_to_string("servers.txt")
        .expect("Invalid File")
        .lines()
        .map(|s_port| s_port.parse::<u32>().expect("Invalid Server Port"))
        .collect();

    println!("Server ports read from file: {:?}", server_ports);

    if !server_ports.contains(&port) {
        eprintln!("Own port {} not found in servers.txt", port);
        std::process::exit(1);
    }

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server running on 127.0.0.1:{}", port);

    let mut connected_servers = HashSet::new();
    let up_servers = (server_ports.len() + 1)/2;
    connected_servers.insert(port);

    while connected_servers.len() < up_servers {
        for &server_port in &server_ports {

            // Attempt to connect to the server
            match TcpStream::connect(format!("127.0.0.1:{}", server_port)).await {
                Ok(_) => {
                    println!("Connected to server on port {}", server_port);
                    connected_servers.insert(server_port);
                }
                Err(e) => {
                    println!("Failed to connect to server on port {}: {}", server_port, e);
                }
            }

            // Pause briefly before trying the next server
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    println!("Server {} Connected", port);

    /*let raft_node = RaftNode::new(port as u64, port, listener);

    if raft_node.start_election(&server_ports, port as u32).await {
        println!("Server on port {} is the leader", port);
    } else {
        println!("Server on port {} is not the leader", port);
    }

    // Spawn a new task to handle each connection
    let stream:TcpStream = raft_node.stream().await;
    tokio::spawn(async move {
        if let Err(e) = raft_node.process(stream).await {
            eprintln!("Failed to process connection: {}", e);
        }
    });*/
    loop {
    }
    Ok(())
}

