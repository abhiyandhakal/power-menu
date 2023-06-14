// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, process::Command};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn shutdown() {
    Command::new("poweroff")
        .spawn()
        .expect("failed to power off");
}

#[tauri::command]
fn logout() {
    Command::new("loginctl")
        .arg("kill-user")
        .arg("$USER")
        .spawn()
        .expect("Failed to log out");
}

#[tauri::command]
fn reboot() {
    Command::new("reboot").output().expect("Failed to reboot");
}

#[tauri::command]
fn close() {
    std::process::exit(1)
}

#[tauri::command]
fn suspend() {
    Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Failed to suspend");
}

fn main() {
    // check if power menu is open
    let all_processes = Command::new("ps")
        .arg("aux")
        .output()
        .expect("Failed to execute the command ps aux");

    let is_running = String::from_utf8(all_processes.stdout)
        .expect("Failed to convert to string.")
        .contains("power-menu");

    if is_running {}

    let quit = CustomMenuItem::new("quit", "Quit");
    let hide = CustomMenuItem::new("hide", "Hide");
    let show = CustomMenuItem::new("show", "Show");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(show);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().expect("Failed to hide the window");
                api.prevent_close();
            }

            _ => {}
        })
        .on_page_load(|window, _| {
            window.hide().expect("Failed to hide the window");
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").expect("Failed to get main window");
                    window.hide().expect("Failed to hide the window");
                }
                "show" => {
                    let window = app.get_window("main").expect("Failed to get main window");
                    window.show().expect("Failed to show the window");
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            shutdown, logout, reboot, suspend, close,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
