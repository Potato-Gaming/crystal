#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use dotenv::dotenv;
use std::env;
use std::process::Command;
use std::thread;

mod cmd;

fn main() {
  dotenv().ok();
  start_crystal_bin();

  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}

fn start_crystal_bin() {
  let binary_path = env::var("CRYSTAL_CORE_BIN");
  let binary_path = match binary_path {
    Ok(b) => b,
    Err(_) => {
      println!("CRYSTAL_CORE_BIN not found, make sure crystal-core is running");
      return;
    }
  };

  thread::spawn(move || {
    let mut child = Command::new(binary_path)
      .spawn()
      .expect("Failed to spawn crystal-core");
    let ecode = child.wait().expect("Failed to wait on crystal-core");
    println!("Succeeded: {}", ecode.success());
  });
}
