use std::{collections::HashMap, error::Error};

use signal_hook::{consts::SIGUSR1, iterator::Signals};
use wayfire_rs::ipc::WayfireSocket;

/// This little program is used to toggle the opacity
/// (alpha value) of windows (views) in Wayfire
///
/// After running it, trigger the toggle by sending SIGUSR1, like this:
/// `kill -n 10 $(pidof wayfire-toggle-active-alpha)`
/// If this doesn't work, double check with `kill -l` that SIGUSR1 is 10

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = WayfireSocket::connect().await?;

    let mut defaults: HashMap<i64, f64> = HashMap::new();

    for _ in Signals::new([SIGUSR1])?.forever() {
        let Ok(view) = socket.get_focused_view().await else {
            continue;
        };

        let Ok(alpha) = socket.get_view_alpha(view.id).await.map(|v| v.alpha) else {
            continue;
        };

        let default_alpha = match defaults.get(&view.id) {
            Some(default) => *default,
            None => {
                defaults.insert(view.id, alpha);
                alpha
            }
        };

        let new_alpha = match default_alpha == alpha {
            true => 1.,
            false => default_alpha,
        };

        let Ok(_) = socket.set_view_alpha(view.id, new_alpha).await else {
            eprintln!("Failed to toggle alpha");
            continue;
        };
    }

    Ok(())
}
