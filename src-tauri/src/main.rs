// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn shutdown() {
    Command::new("poweroff")
        .spawn()
        .expect("failed to power off");
}

#[tauri::command]
fn logout() {
    Command::new("killall")
        .arg("Xorg")
        .spawn()
        .expect("Failed to log out");
}

#[tauri::command]
fn reboot() {
    Command::new("reboot").output().expect("Failed to reboot");
}

#[tauri::command]
fn suspend() {
    Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Failed to reboot");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![shutdown, logout, reboot, suspend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
