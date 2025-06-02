# Wayfire-rs

wayfire-rs is a high-level Rust interface for Wayfire, offering comprehensive control over the wayfire compositor through its IPC protocol.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Installation

To include `wayfire-rs` in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
wayfire-rs = "0.2.1"
```

### Usage

Basic usage in wayfire-rs/examples folder and lots of examples in wayfire-rs/src/main.rs

## Wayfire IPC API Reference

### View Operations
- **`get_view`** - Retrieves information about a specific view
- **`get_focused_view`** - Gets the currently focused view
- **`get_view_alpha`** - Retrieves the transparency (alpha) value of a view
- **`set_view_alpha`** - Sets the transparency (alpha) value of a view
- **`set_view_always_on_top`** - Toggles "always on top" state for a view
- **`set_view_fullscreen`** - Toggles fullscreen mode for a view
- **`set_view_sticky`** - Toggles sticky (visible on all workspaces) state
- **`set_view_minimized`** - Minimizes or restores a view
- **`send_view_to_back`** - Sends a view to the back of the stacking order
- **`close_view`** - Closes a view
- **`configure_view`** - Adjusts view geometry (position/size) and output assignment
- **`set_focus`** - Focuses a specific view
- **`assign_slot`** - Assigns view to a grid slot (e.g., "grid/left")

### Output Management
- **`get_output`** - Retrieves information about a specific output
- **`get_focused_output`** - Gets the currently focused output
- **`create_headless_output`** - Creates a virtual output with specified dimensions
- **`destroy_headless_output`** - Removes a headless output (by ID or name)

### Workspace & Layout
- **`get_tiling_layout`** - Retrieves layout for a workspace
- **`set_tiling_layout`** - Configures workspace layout
- **`set_workspace`** - Moves view to specific workspace on an output
- **`wset_info`** - Gets workspace set information
- **`toggle_showdesktop`** - Toggles show-desktop mode (minimizes/restores all views)

### Effects & Animations
- **`expo_toggle`** - Toggles workspace overview (Expo)
- **`scale_toggle`** - Toggles window overview (Scale)
- **`scale_toggle_all`** - Toggles Scale for all windows
- **`cube_activate`** - Activates 3D workspace cube
- **`cube_rotate_left`** - Rotates cube left
- **`cube_rotate_right`** - Rotates cube right

### Input & Cursor
- **`get_cursor_position`** - Retrieves current (x,y) cursor coordinates
- **`configure_input_device`** - Enables/disables input devices
- **`get_keyboard_layout`** - Gets the current keyboard layout information
- **`set_keyboard_layout`** - Sets the active keyboard layout (e.g., by index or name)


### Event System
- **`watch`** - Subscribes to window-rules events (with optional filter)

### Configuration
- **`get_configuration`** - Retrieves Wayfire's full configuration
- **`get_option_value`** - Gets value of a specific config option

### Contributing

If you want to contribute to the wayfire-rs project, follow these steps:

    Fork the repository.
    Create a new branch for your feature or bug fix.
    Make your changes and test them.
    Submit a pull request with a detailed description of your changes.

### License

```
wayfire-rs is licensed under the MIT License.

```
