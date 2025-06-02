use std::error::Error;
use wayfire_rs::ipc::WayfireSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = WayfireSocket::connect().await?;

    // Get current keyboard layout
    let current_layout_index = match socket.get_keyboard_layout().await {
        Ok(layout) => {
            let index = layout["layout_index"].as_u64().unwrap_or(0);
            println!("Current layout index: {}", index);
            index
        }
        Err(e) => {
            eprintln!("Failed to get keyboard layout: {}", e);
            return Ok(());
        }
    };

    // Set the same layout so it won't change the current layout
    match socket.set_keyboard_layout(current_layout_index as u32).await {
        Ok(_) => println!("Restored layout to index {}", current_layout_index),
        Err(e) => eprintln!("Failed to set layout: {}", e),
    }

    Ok(())
}
