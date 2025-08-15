use wayfire_rs::ipc::{WayfireSocket, View};
use rand::Rng; // Add rand = "0.8" to Cargo.toml

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Wayfire IPC socket
    let mut socket = WayfireSocket::connect().await?;

    // Get the focused view
    let focused_view: View = socket.get_focused_view().await?;
    let view_id = focused_view.id;
    println!("Focused view ID: {}", view_id);

    // Generate random workspace coordinates
    let mut rng = rand::thread_rng();
    let target_x: i32 = rng.gen_range(0..3); // adjust max as needed
    let target_y: i32 = rng.gen_range(0..3);

    println!(
        "Moving view {} to workspace ({}, {})",
        view_id, target_x, target_y
    );

    // Send the view to the random workspace
    match socket.send_view_to_workspace(view_id as i32, target_x, target_y).await {
        Ok(_) => println!("Sent view to workspace successfully."),
        Err(e) => eprintln!("Failed to send view to workspace: {}", e),
    }

    Ok(())
}

