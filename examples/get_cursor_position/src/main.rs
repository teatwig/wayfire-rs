use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

async fn print_cursor_position(socket: &mut WayfireSocket) {
    match socket.get_cursor_position().await {
        Ok((x, y)) => println!("Cursor position: {}, {}", x, y),
        Err(e) => eprintln!("Failed to get cursor position: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = WayfireSocket::connect().await?;
    print_cursor_position(&mut socket).await;

    Ok(())
}
