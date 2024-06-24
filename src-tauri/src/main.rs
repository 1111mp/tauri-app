// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use utils::resolve;

mod cmds;
mod utils;

fn main() -> tauri::Result<()> {
    let builder = tauri::Builder::default()
        .setup(|app| {
            resolve::resolve_setup(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmds::greet]);

    let app = builder.build(tauri::generate_context!())?;

    app.run(|app_handle, e| match e {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                match event {
                    tauri::WindowEvent::Destroyed => {
                        // Destroyed Event
                    }
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        // CloseRequested Event
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });

    Ok(())
}
