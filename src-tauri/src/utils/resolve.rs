use tauri::{App, AppHandle, Manager};

/// handle something when start app
pub fn resolve_setup(app: &mut App) -> tauri::Result<()> {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    create_window(&app.app_handle())?;

    Ok(())
}

/// create main window
pub fn create_window(app_handle: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app_handle.get_window("main") {
        window.unminimize()?;
        window.show()?;
        window.is_focused()?;
        return Ok(());
    }

    let builder = tauri::WindowBuilder::new(
        app_handle,
        "main",
        tauri::WindowUrl::App("index.html".into()),
    )
    .title("Tauri App")
    .visible(false)
    .fullscreen(false)
    .inner_size(1024.0, 728.0)
    .resizable(false)
    .center();

    #[cfg(target_os = "windows")]
    builder
        .decorations(true)
        .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
        .transparent(true)
        .visible(false)
        .build()?;

    Ok(())
}
