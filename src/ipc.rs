use crate::models::{
    InputDevice, Layout, MsgTemplate, OptionValueResponse, Output, View, ViewAlpha,
    WayfireConfiguration, WorkspaceSet,
};
use serde_json::Value;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream as TokioUnixStream;

pub struct WayfireSocket {
    client: TokioUnixStream,
    pending_events: VecDeque<Value>,
}

impl WayfireSocket {
    pub async fn connect() -> io::Result<Self> {
        let socket_name =
            env::var("WAYFIRE_SOCKET").expect("WAYFIRE_SOCKET environment variable not set");
        let client = TokioUnixStream::connect(&socket_name).await?;
        Ok(WayfireSocket {
            client,
            pending_events: VecDeque::new(),
        })
    }

    pub async fn send_json(&mut self, msg: &MsgTemplate) -> io::Result<Value> {
        let data = serde_json::to_vec(msg)?;
        let header = (data.len() as u32).to_le_bytes();

        self.client.write_all(&header).await?;
        self.client.write_all(&data).await?;

        loop {
            let msg = self.read_message().await?;
            if msg.get("event").is_some() {
                self.pending_events.push_back(msg);
                continue;
            } else {
                return Ok(msg);
            }
        }
    }

    pub async fn read_exact(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; n];
        self.client.read_exact(&mut buf).await?;
        Ok(buf)
    }

    pub async fn read_message(&mut self) -> io::Result<Value> {
        let len_buf = self.read_exact(4).await?;
        let len = u32::from_le_bytes(len_buf.try_into().unwrap()) as usize;

        let response_buf = self.read_exact(len).await?;
        let response: Value = serde_json::from_slice(&response_buf)?;

        if response.get("error").is_some() {
            eprintln!("Error: {:?}", response);
        }

        Ok(response)
    }

    pub async fn read_next_event(&mut self) -> io::Result<Value> {
        match self.pending_events.pop_front() {
            Some(event) => Ok(event),
            None => self.read_message().await,
        }
    }

    pub async fn list_views(&mut self) -> io::Result<Vec<View>> {
        let message = MsgTemplate {
            method: "window-rules/list-views".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let views: Vec<View> = serde_json::from_value(response)?;

        Ok(views)
    }

    pub async fn list_outputs(&mut self) -> io::Result<Vec<Output>> {
        let message = MsgTemplate {
            method: "window-rules/list-outputs".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let outputs: Vec<Output> = serde_json::from_value(response)?;

        Ok(outputs)
    }

    pub async fn list_wsets(&mut self) -> io::Result<Vec<WorkspaceSet>> {
        let message = MsgTemplate {
            method: "window-rules/list-wsets".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let workspace_sets: Vec<WorkspaceSet> = serde_json::from_value(response)?;

        Ok(workspace_sets)
    }

    pub async fn list_input_devices(&mut self) -> io::Result<Vec<InputDevice>> {
        let message = MsgTemplate {
            method: "input/list-devices".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let input_devices: Vec<InputDevice> = serde_json::from_value(response)?;

        Ok(input_devices)
    }

    pub async fn get_configuration(&mut self) -> io::Result<WayfireConfiguration> {
        let message = MsgTemplate {
            method: "wayfire/configuration".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;
        let configuration: WayfireConfiguration = serde_json::from_value(response)?;

        Ok(configuration)
    }

    pub async fn get_option_value(&mut self, option: &str) -> io::Result<OptionValueResponse> {
        let message = MsgTemplate {
            method: "wayfire/get-config-option".to_string(),
            data: Some(serde_json::json!({
                "option": option
            })),
        };

        let response = self.send_json(&message).await?;
        let option_value_response: OptionValueResponse = serde_json::from_value(response)?;

        Ok(option_value_response)
    }

    pub async fn get_output(&mut self, output_id: i64) -> io::Result<Output> {
        let message = MsgTemplate {
            method: "window-rules/output-info".to_string(),
            data: Some(serde_json::json!({
                "id": output_id
            })),
        };

        let response = self.send_json(&message).await?;
        let output: Output = serde_json::from_value(response)?;

        Ok(output)
    }

    #[allow(dead_code)]
    pub async fn get_view(&mut self, view_id: i64) -> io::Result<View> {
        let message = MsgTemplate {
            method: "window-rules/view-info".to_string(),
            data: Some(serde_json::json!({
                "id": view_id
            })),
        };

        let response = self.send_json(&message).await?;

        let info = response.get("info").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Missing 'info' field in response")
        })?;

        let view: View = serde_json::from_value(info.clone())?;

        Ok(view)
    }

    pub async fn get_focused_view(&mut self) -> Result<View, Box<dyn Error>> {
        let message = MsgTemplate {
            method: "window-rules/get-focused-view".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;

        let view_info = response.get("info").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Missing 'info' field in response")
        })?;

        let view: View = serde_json::from_value(view_info.clone())?;

        Ok(view)
    }

    #[allow(dead_code)]
    pub async fn get_focused_output(&mut self) -> Result<Output, Box<dyn Error>> {
        let message = MsgTemplate {
            method: "window-rules/get-focused-output".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;

        let output_info = response.get("info").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Missing 'info' field in response")
        })?;

        let output: Output = serde_json::from_value(output_info.clone())?;

        Ok(output)
    }

    pub async fn get_cursor_position(&mut self) -> io::Result<(f64, f64)> {
        let message = MsgTemplate {
            //FIXME: Create a Wayfire PR to modify from _ to `get-cursor-position` to maintain the pattern.
            method: "window-rules/get_cursor_position".to_string(),
            data: None,
        };

        let response = self.send_json(&message).await?;

        let pos = response.get("pos").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing 'pos' field in response",
            )
        })?;

        Ok((
            pos["x"].as_f64().unwrap_or(0.0),
            pos["y"].as_f64().unwrap_or(0.0),
        ))
    }

    pub async fn get_view_alpha(&mut self, view_id: i64) -> io::Result<ViewAlpha> {
        let message = MsgTemplate {
            method: "wf/alpha/get-view-alpha".to_string(),
            data: Some(serde_json::json!({
                "view-id": view_id
            })),
        };

        let response = self.send_json(&message).await?;

        let view_alpha: ViewAlpha = serde_json::from_value(response).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse response: {}", e),
            )
        })?;

        Ok(view_alpha)
    }

    #[allow(dead_code)]
    pub async fn set_view_alpha(&mut self, view_id: i64, alpha: f64) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wf/alpha/set-view-alpha".to_string(),
            data: Some(serde_json::json!({
                "view-id": view_id,
                "alpha": alpha
            })),
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn get_tiling_layout(&mut self, wset: i64, x: i64, y: i64) -> io::Result<Layout> {
        let message = MsgTemplate {
            method: "simple-tile/get-layout".to_string(),
            data: Some(serde_json::json!({
                "wset-index": wset,
                "workspace": {
                    "x": x,
                    "y": y
                }
            })),
        };

        let response = self.send_json(&message).await?;

        let layout_value = response.get("layout").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Missing `layout` field in response",
            )
        })?;

        let layout: Layout = serde_json::from_value(layout_value.clone())?;

        Ok(layout)
    }

    #[allow(dead_code)]
    pub async fn set_tiling_layout(
        &mut self,
        wset: i64,
        x: i64,
        y: i64,
        layout: &Layout,
    ) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "simple-tile/set-layout".to_string(),
            data: Some(serde_json::json!({
                "wset-index": wset,
                "workspace": {
                    "x": x,
                    "y": y
                },
                "layout": layout
            })),
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn set_view_always_on_top(&mut self, view_id: i64, state: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/set-always-on-top".to_string(),
            data: Some(serde_json::json!({
                "view_id": view_id,
                "state": state
            })),
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn set_view_fullscreen(&mut self, view_id: i64, state: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/set-fullscreen".to_string(),
            data: Some(serde_json::json!({
                "view_id": view_id,
                "state": state
            })),
        };

        self.send_json(&message).await
    }

    pub async fn expo_toggle(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "expo/toggle".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    pub async fn scale_toggle(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "scale/toggle".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn scale_toggle_all(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "expo/toggle_all".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    pub async fn cube_activate(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "cube/activate".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    pub async fn cube_rotate_left(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "cube/rotate_left".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    pub async fn cube_rotate_right(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "cube/rotate_right".to_string(),
            data: None,
        };

        self.send_json(&message).await
    }

    pub async fn toggle_showdesktop(&mut self) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/toggle_showdesktop".to_string(),
            data: None,
        };
        self.send_json(&message).await
    }

    pub async fn set_view_sticky(&mut self, view_id: i64, state: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/set-sticky".to_string(),
            data: Some(serde_json::json!({
                "view_id": view_id,
                "state": state,
            })),
        };
        self.send_json(&message).await
    }

    pub async fn send_view_to_back(&mut self, view_id: i64, state: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/send-to-back".to_string(),
            data: Some(serde_json::json!({
                "view_id": view_id,
                "state": state,
            })),
        };
        self.send_json(&message).await
    }

    pub async fn set_view_minimized(&mut self, view_id: i64, state: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wm-actions/set-minimized".to_string(),
            data: Some(serde_json::json!({
                "view_id": view_id,
                "state": state,
            })),
        };
        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn configure_input_device(&mut self, id: i64, enabled: bool) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "input/configure-device".to_string(),
            data: Some(serde_json::json!({
                "id": id,
                "enabled": enabled,
            })),
        };
        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn close_view(&mut self, view_id: i64) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "window-rules/close-view".to_string(),
            data: Some(serde_json::json!({
                "id": view_id,
            })),
        };
        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn wset_info(&mut self, id: i64) -> io::Result<serde_json::Value> {
        let message = MsgTemplate {
            method: "window-rules/wset-info".to_string(),
            data: Some(serde_json::json!({
                "id": id,
            })),
        };

        self.send_json(&message).await
    }

    pub async fn watch(&mut self, events: Option<Vec<String>>) -> io::Result<serde_json::Value> {
        let mut data = serde_json::json!({});
        if let Some(events) = events {
            data["events"] = serde_json::json!(events);
        }

        let message = MsgTemplate {
            method: "window-rules/events/watch".to_string(),
            data: Some(data),
        };

        self.send_json(&message).await
    }

    pub async fn configure_view(
        &mut self,
        view_id: i64,
        x: i64,
        y: i64,
        w: i64,
        h: i64,
        output_id: Option<i64>,
    ) -> io::Result<serde_json::Value> {
        let mut data = serde_json::json!({
            "id": view_id,
            "geometry": {
                "x": x,
                "y": y,
                "width": w,
                "height": h
            }
        });

        if let Some(output_id) = output_id {
            data["output_id"] = serde_json::json!(output_id);
        }

        let message = MsgTemplate {
            method: "window-rules/configure-view".to_string(),
            data: Some(data),
        };

        self.send_json(&message).await
    }

    pub async fn assign_slot(&mut self, view_id: i64, slot: &str) -> io::Result<serde_json::Value> {
        let message = MsgTemplate {
            method: format!("grid/{}", slot),
            data: Some(serde_json::json!({
                "view_id": view_id
            })),
        };

        self.send_json(&message).await
    }

    pub async fn set_focus(&mut self, view_id: i64) -> io::Result<serde_json::Value> {
        let message = MsgTemplate {
            method: "window-rules/focus-view".to_string(),
            data: Some(serde_json::json!({
                "id": view_id
            })),
        };

        self.send_json(&message).await
    }

    pub async fn set_workspace(
        &mut self,
        x: i64,
        y: i64,
        view_id: i64,
        output_id: i64,
    ) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "vswitch/set-workspace".to_string(),
            data: Some(serde_json::json!({
                "x": x,
                "y": y,
                "output-id": output_id,
                "view-id": view_id
            })),
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn create_headless_output(&mut self, width: u32, height: u32) -> io::Result<Value> {
        let message = MsgTemplate {
            method: "wayfire/create-headless-output".to_string(),
            data: Some(serde_json::json!({
                "width": width,
                "height": height
            })),
        };

        self.send_json(&message).await
    }

    #[allow(dead_code)]
    pub async fn destroy_headless_output(
        &mut self,
        output_name: Option<String>,
        output_id: Option<i64>,
    ) -> io::Result<Value> {
        let mut data = serde_json::json!({});

        if let Some(name) = output_name {
            data["output"] = serde_json::Value::String(name);
        } else if let Some(id) = output_id {
            data["output-id"] = serde_json::Value::Number(serde_json::Number::from(id));
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Either output_name or output_id must be provided",
            ));
        }

        let message = MsgTemplate {
            method: "wayfire/destroy-headless-output".to_string(),
            data: Some(data),
        };

        self.send_json(&message).await
    }
}
