use std::env;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    // Parse the port number
    let port = match args[1].parse::<u32>() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number");
            std::process::exit(1);
        }
    };

    // Bind the server to the specified port
    let bind_addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(bind_addr).await?;
    println!("Server running on 127.0.0.1:{}", port);

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

// Function to handle each connection
async fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    if n == 0 {
        return Ok(());
    }

    // Print received message (for debugging purposes)
    println!("Received message: {}", String::from_utf8_lossy(&buf[..n]));

    // Write response back to the client
    stream.write_all(b"nice").await?;
    Ok(())
}

