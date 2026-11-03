use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::atomic::AtomicUsize;{AtomicUsize}

// Loadbalancer 
struct LoadBalancer {
    backends: Vec<String>,
    current_index: AtomicUsize,
}

impl LoadBalancer {
    fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            current_index: AtomicUsize::new(0)
        }
    }

    fn next_backend(&self) -> String {

        let index = self.current_index.fetch_add(1, Ordering::Relaxed);
        self.backends[index % self.backends.len()].clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    let listen_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(listen_address).await?;



    println!("Server is running on {listen_address}");

    let backends = vec![
        "127.0.0.1:9001".to_string(),
        "127.0.0.1:9002".to_string(),
    ];

    let balancer = Arc::new(LoadBalancer::new(backends));

    loop {
        let (client_stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let balancer = Arc::clone(&balancer);

        tokio::spawn(
            async move{
                match handle_client(client_stream, balancer).await {
                    Ok(_) => {}
                    Err(e) => println!("Error handling client: {}", e),
                }
            }
        );
    }
}

async fn handle_client(mut stream: TcpStream, 
balancer: Arc<LoadBalancer>) -> Result<(), Box<dyn std::error::Error>> {

    let target = balancer.next_backend();
    println!("Forwarding to {}", target);
    
    let mut upstream = TcpStream::connect(&target).await?;

    tokio::io::copy_bidirectional(& mut stream, &mut upstream).await?;
    Ok(())
}
