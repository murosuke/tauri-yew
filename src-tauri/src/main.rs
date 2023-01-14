#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Serialize, Deserialize};
use std::net::TcpStream;
// use std::str::FromStr;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use tauri::Manager;
// use toml::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestTomlTop {
    ui_access: UiAccess,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UiAccess {
    scheme: String,
    url: String,
    port: String,
}

fn check_connect(addr: &str) -> bool {
    if let Ok(_stream) = TcpStream::connect(addr) {
        println!("Connected to the server!");
        true
    } else {
        println!("Connect to the server failed!");
        false
    }
}

fn main() {
    // read settings
    let toml_path_str = format!("{}/test.toml", env::current_exe().unwrap().parent().unwrap().to_string_lossy());
    let toml_path = std::path::Path::new(&toml_path_str);
    if !std::path::Path::new(&toml_path).exists() {
        println!("test.toml not found! create default settings.");
        let default = TestTomlTop{ui_access: UiAccess{scheme:"http".to_string(), url:"127.0.0.1".to_string(), port:"8080".to_string()}};
        let mut file = File::create(toml_path).unwrap();
        write!(file, "{}", toml::to_string(&default).unwrap()).unwrap();
        file.flush().unwrap();
        
    }
    let toml_str = fs::read_to_string(toml_path).unwrap();
    let toml_setting: TestTomlTop = toml::from_str(&toml_str).unwrap();
    // println!("{:?}", toml_setting);

    tauri::Builder::default()
        .setup(move |app| {
            let addr: String = format!(
                "{}:{}",
                toml_setting.ui_access.url, toml_setting.ui_access.port
            );
            let window = app.get_window("main").unwrap();
            if check_connect(addr.as_str()) {
                let schema: String = format!("{}://", toml_setting.ui_access.scheme);
                println!("loading {}{}", schema, addr);
                window
                    .eval(&format!("window.location.replace('{}{}')", schema, addr))
                    .unwrap();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
