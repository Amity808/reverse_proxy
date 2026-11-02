use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    let listen_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(listen_address).await?;

    println!("Server is running on {listen_address}");

    loop {
        let (client_stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(
            async move{
                match handle_client(client_stream).await {
                    Ok(_) => {}
                    Err(e) => println!("Error handling client: {}", e),
                }
            }
        );
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut upstream = TcpStream::connect("127.0.0.1:9000").await?;

    tokio::io::copy_bidirectional(& mut stream, &mut upstream).await?;
    Ok(())
}
