// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn shutdown() {
    Command::new("poweroff")
        .spawn()
        .expect("failed to power off");
    close();
}

#[tauri::command]
fn logout() {
    Command::new("loginctl")
        .arg("kill-user")
        .arg("$USER")
        .spawn()
        .expect("Failed to log out");
    close();
}

#[tauri::command]
fn reboot() {
    Command::new("reboot").output().expect("Failed to reboot");
    close();
}

#[tauri::command]
fn close() {
    Command::new("killall")
        .arg("power-menu")
        .spawn()
        .expect("Failed to exit power-menu");
}

#[tauri::command]
fn suspend() {
    Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Failed to reboot");
    close();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            shutdown, logout, reboot, suspend, close
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
