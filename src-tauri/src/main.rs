// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs::OpenOptions, process::Command};

use serde_json::Value;
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
    Command::new("reboot").spawn().expect("Failed to reboot");
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

#[tauri::command]
fn check_root() {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to check sudo");

    let is_root = String::from_utf8(output.stdout).expect("Failed to convert to string.");

    if is_root == "root" {
        println!("is root");
    } else {
        println!("not root");
    }
}

#[tauri::command]
fn get_config() -> Value {
    // get home path
    let home = env::var("HOME").expect("Failed to get home path");

    let default_config = r#"
    {
        "warn": true
    }
        "#;

    let default_config: Value = serde_json::from_str(default_config).expect("Failed to parse json");

    let mut config = default_config.clone();

    let config_file = OpenOptions::new()
        .read(true)
        .open(format!("{}/.config/power-menu/config.json", home));

    match config_file {
        Ok(file) => {
            config = serde_json::from_reader(file).expect("Failed to parse config file");
        }
        Err(_) => {
            println!("no config file found");
        }
    }

    config
}

// fn set_config(config: Value) {
//     // get home path
//     let home = env::var("HOME").expect("Failed to get home path");
//
//     let config_file = OpenOptions::new()
//         .write(true)
//         .create(true)
//         .open(format!("{}/.config/power-menu/config.json", home));
//
//     match config_file {
//         Ok(mut file) => {
//             serde_json::to_writer_pretty(file, &config).expect("Failed to write to config file");
//         }
//         Err(_) => {
//             println!("no config file found");
//         }
//     }
// }

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

    let menu_items = vec![
        ("show-hide", "Show / Hide"),
        ("shutdown", "Shutdown"),
        ("suspend", "Suspend"),
        ("logout", "Logout"),
        ("reboot", "Reboot"),
        ("quit", "Quit"),
    ];

    let tray_menu = menu_items
        .into_iter()
        .fold(SystemTrayMenu::new(), |menu, (name, label)| {
            menu.add_item(CustomMenuItem::new(name, label))
                .add_native_item(tauri::SystemTrayMenuItem::Separator)
        });

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
                "show-hide" => {
                    let window = app.get_window("main").expect("Failed to get main window");

                    if window
                        .is_visible()
                        .expect("Failed to check if window is visible")
                    {
                        window.hide().expect("Failed to hide the window");
                    } else {
                        window.show().expect("Failed to show the window");
                    }
                }
                // "shutdown" => shutdown(),
                "shutdown" => {
                    app.get_window("settings")
                        .expect("Failed to get new window")
                        .show()
                        .expect("Failed to show new window");
                }
                "suspend" => suspend(),
                "logout" => logout(),
                "reboot" => reboot(),
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            app.listen_global("settings", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            shutdown, logout, reboot, suspend, close, get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
