#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crossbeam::channel::{unbounded, Receiver, Sender};
use crystal_core::handlers::get_current_summoner;
use crystal_core::LOCKFILE;
use crystal_core::{events, lockfile};
use dotenv::dotenv;
use std::thread;
use tauri::WebviewMut;

mod cmd;

fn main() {
    #[cfg(target_family = "unix")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    pretty_env_logger::init();

    let (tx_ws, rx_ws): (Sender<events::LeagueEvent>, Receiver<events::LeagueEvent>) = unbounded();
    bootstrap_crystal_core(tx_ws);

    tauri::AppBuilder::new()
        .setup(move |webview, _source| {
            let webview = webview.clone().as_mut();
            let rx_ws = rx_ws.clone();

            emit_league_events(rx_ws, webview);
        })
        .invoke_handler(|webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        CurrentSummoner { callback, error } => tauri::execute_promise(
                            webview,
                            || {
                                let summoner = get_current_summoner(&LOCKFILE).unwrap();

                                Ok(summoner)
                            },
                            callback,
                            error,
                        ),
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}

fn bootstrap_crystal_core(tx_ws: Sender<events::LeagueEvent>) {
    let (tx, rx): (
        Sender<events::LockfileEvent>,
        Receiver<events::LockfileEvent>,
    ) = unbounded();

    lockfile::watch_lockfile(&LOCKFILE, tx).unwrap();
    events::listen(&LOCKFILE, rx, tx_ws);
}

fn emit_league_events(rx: Receiver<events::LeagueEvent>, webview: WebviewMut) {
    thread::spawn(move || {
        let rx = rx;
        let mut webview = webview;

        loop {
            let event = rx.recv().unwrap();
            if event.to_string() == String::from("NotTracked") {
                continue;
            }
            println!("Sending event: {}", event.to_string());
            tauri::event::emit(&mut webview, event.to_string(), Some(event)).unwrap();
        }
    });
}
