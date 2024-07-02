use std::env;
use std::fs;
use std::collections::HashSet;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    while connected_servers.len() < 3 {
        for server_port in &server_ports {
            // Skip own port
            if *server_port == port{
                continue;
            }

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

    // Accept connections in a loop
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        // Spawn a new task to handle each connection
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                eprintln!("Failed to process connection: {}", e);
            }
        });
    }
}

async fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    if n == 0 {
        return Ok(());
    }

    let message = String::from_utf8_lossy(&buf[..n]).to_string();
    let tokens: Vec<String> = message.split_whitespace().map(|s| s.to_string()).collect();

    if tokens.len() < 1 || tokens.len() > 3 {
        stream.write_all(b"invalid").await?;
        return Ok(());
    }

    match tokens[0].as_str() {
        "R" | "D" => {
            if tokens.len() != 2 {
                stream.write_all(b"invalid").await?;
            } else {
                if let Ok(value) = tokens[1].parse::<i32>() {
                    println!("Received {} command with value: {}",tokens[0], value);
                    stream.write_all(b"command received").await?;
                } else {
                    stream.write_all(b"invalid").await?;
                }
            }
        }
        "W" => {
            if tokens.len() != 3 {
                stream.write_all(b"invalid").await?;
            } else {
                // Handle "W" command
                if let (Ok(value1), Ok(value2)) = (tokens[1].parse::<i32>(), tokens[2].parse::<i32>()) {
                    // Do something with value1 and value2
                    println!("Received W command with values: {} {}", value1, value2);
                    stream.write_all(b"command received").await?;
                } else {
                    stream.write_all(b"invalid").await?;
                }
            }
        }
        _ => {
            stream.write_all(b"invalid").await?;
        }
    }

    Ok(())
}
