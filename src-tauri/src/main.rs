// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    fs::{create_dir_all, File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

use serde_json::{json, Value};
use tauri::{App, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

// tauri commands
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
    let default_config: Value = json!( {
        "warn": true
    });

    let mut config = default_config.clone();

    let config_file = OpenOptions::new().read(true).open(get_config_file());

    match config_file {
        Ok(file) => config = serde_json::from_reader(file).expect("Failed to parse config file"),
        Err(_) => println!("no config file found"),
    }

    config
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

    let menu_items = vec![
        ("show-hide", "Show / Hide"),
        ("settings", "Settings"),
        ("shutdown", "Shutdown"),
        ("suspend", "Suspend"),
        ("logout", "Logout"),
        ("reboot", "Restart"),
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
                "quit" => std::process::exit(0),
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
                "settings" => app
                    .get_window("settings")
                    .expect("Failed to get settings window")
                    .show()
                    .expect("Failed to show settings window"),
                "shutdown" => power_action(app, "shutdown"),
                "suspend" => power_action(app, "suspend"),
                "logout" => power_action(app, "logout"),
                "reboot" => power_action(app, "reboot"),
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            app.listen_global("settings", |event| {
                if let Some(config_str) = event.payload() {
                    let config: Value = serde_json::from_str(config_str).expect("Failed to parse");

                    if let Some(config) = config.get("message") {
                        set_config(config.to_string());
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            shutdown, logout, reboot, suspend, close, get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_config_dir() -> PathBuf {
    match env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir).join("power-menu"),
        None => {
            let config_path: PathBuf = dirs::home_dir()
                .expect("Failed to get home path")
                .join(".config")
                .join("power-menu");

            config_path
        }
    }
}

fn get_config_file() -> PathBuf {
    get_config_dir().join("config.json")
}

fn set_config(config: String) {
    let config_file_path = get_config_file();

    let mut config_file = if config_file_path.exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(get_config_file())
            .expect("Failed to get config file")
    } else {
        println!("path: {:?}", config_file_path);

        if get_config_dir().is_dir() {
            File::create(config_file_path).expect("Failed to create config file")
        } else {
            create_dir_all(get_config_dir()).expect("Failed to create config dir");
            File::create(config_file_path).expect("Failed to create config file")
        }
    };

    config_file
        .write_all(config.as_bytes())
        .expect("Failed to write to config file");
}

fn power_action(app: &AppHandle, action: &str) {
    let config: Value = get_config();

    if let Some(warn) = config.get("warn") {
        if warn.as_bool().unwrap_or(true) {
            let warning_window = match action {
                "shutdown" => "shutdown_warning",
                "suspend" => "suspend_warning",
                "logout" => "logout_warning",
                "reboot" => "reboot_warning",
                _ => "",
            };

            app.get_window(warning_window)
                .expect("Failed to get warning window")
                .show()
                .expect("Failed to show warning window");
        } else {
            match action {
                "shutdown" => shutdown(),
                "suspend" => suspend(),
                "logout" => logout(),
                "reboot" => reboot(),
                _ => {}
            }
        }
    }
}
