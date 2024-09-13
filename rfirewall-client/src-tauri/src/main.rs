// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    process::Command,
    result::Result,
    sync::{Arc, Mutex},
};

use rfirewall::core::firewall::Firewall;
use tauri::{App, State};

struct AppState {
    firewall: Arc<Mutex<Firewall>>,
}

#[tauri::command]
fn start_firewall() -> Result<(), String> {
    // Use sudo to run the privileged firewall command with the dynamic path
    let output = Command::new("sudo")
        .arg(&get_rfirewall_path())
        .arg("start")
        .output();

    if let Ok(result) = output {
        if result.status.success() {
            return Ok(());
        } else {
            return Err(format!(
                "Failed to start firewall: {}",
                String::from_utf8_lossy(&result.stderr)
            ));
        }
    } else {
        return Err("Failed to start firewall".to_string());
    }
}

#[tauri::command]
fn stop_firewall() -> Result<(), String> {
    let output = Command::new("sudo")
        .arg(&get_rfirewall_path())
        .arg("stop")
        .output();

    if let Ok(result) = output {
        if result.status.success() {
            return Ok(());
        } else {
            return Err(format!(
                "Failed to stop firewall: {}",
                String::from_utf8_lossy(&result.stderr)
            ));
        }
    } else {
        return Err("Failed to stop firewall".to_string());
    }
}

#[tauri::command]
fn add_firewall_rule(
    state: State<AppState>,
    allow: bool,
    port: u16,
    ip: &str,
    protocol: &str,
) -> Result<(), String> {
    let mut firewall = state.firewall.lock().unwrap();

    let rule = rfirewall::core::rule::Rule::new(allow, port, ip, protocol);
    firewall.add_rule(rule);

    Ok(())
}

fn get_rfirewall_path() -> String {
    let rfirewall_path = env::var("RFIREWALL_PATH")
        .unwrap_or_else(|_| "rfirewall/target/debug/rfirewall".to_string());
    rfirewall_path
}

fn main() {
    let firewall = Arc::new(Mutex::new(Firewall::new()));
    tauri::Builder::default()
        .manage(AppState {
            firewall: firewall.clone(),
        })
        .invoke_handler(tauri::generate_handler![
            start_firewall,
            stop_firewall,
            add_firewall_rule
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
