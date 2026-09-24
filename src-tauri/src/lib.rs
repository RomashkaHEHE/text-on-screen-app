#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, WindowEvent,
};

#[tauri::command]
fn set_capture_protected(window: tauri::Window, protected: bool) -> Result<(), String> {
    window.set_content_protected(protected).map_err(|e| e.to_string())
}

#[tauri::command]
fn set_click_through(window: tauri::Window, enabled: bool) -> Result<(), String> {
    window.set_ignore_cursor_events(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_release_url(url: String) -> Result<(), String> {
    let parsed = url::Url::parse(&url).map_err(|e| e.to_string())?;
    if parsed.scheme() != "https" && parsed.scheme() != "http" { return Err("unsupported URL scheme".into()); }
    #[cfg(target_os = "windows")]
    { Command::new("cmd").args(["/C", "start", "", &url]).spawn().map_err(|e| e.to_string())?; }
    #[cfg(target_os = "macos")]
    { Command::new("open").arg(&url).spawn().map_err(|e| e.to_string())?; }
    #[cfg(all(unix, not(target_os = "macos")))]
    { Command::new("xdg-open").arg(&url).spawn().map_err(|e| e.to_string())?; }
    Ok(())
}

fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") { let _ = window.show(); let _ = window.set_focus(); }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_capture_protected, set_click_through, open_release_url])
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "Показать настройки", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Выйти", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;
            TrayIconBuilder::new().menu(&menu).on_menu_event(|app, event| {
                match event.id().as_ref() { "show" => show_window(app), "quit" => app.exit(0), _ => {} }
            }).build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event { api.prevent_close(); let _ = window.hide(); }
        })
        .run(tauri::generate_context!())
        .expect("error while running Text on Screen");
}
