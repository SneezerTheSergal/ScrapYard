// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::{CustomMenuItem, EventHandler, Manager, Menu, Submenu, Window, WindowBuilder, WindowUrl};
use std::fs::*;
use std::io::Write;
use std::path::Path;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {


    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![create_file, get_files, get_file_content, save_file, delete_this_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn create_file(file_name: String) {
    let mut corrected_file_name = file_name.replace(" ", "_");
    println!("entered create file with name: {corrected_file_name}");
    let mut path  = Path::new("src/files/").join(format!("{}.txt",corrected_file_name));
    println!("entered create file with path: {:?}", path);
    println!("entered create file with full path: {:?}", path);
    File::create(path.clone()).expect(format!("expected file {} at {:?}",corrected_file_name, path).as_str());
}

#[tauri::command]
fn get_files() -> Vec<String> {
    let entries = fs::read_dir("src/files").unwrap();
    entries.filter_map(|entry| {
        entry.ok().and_then(|e| {
            e.file_name().to_str().map(String::from)
        })
    }).collect()
}

#[tauri::command]
fn get_file_content(file_name: String) -> String {
    let mut path  = Path::new("src/files/").join(format!("{}",file_name));
    println!("{file_name}");
    let result = read_to_string(path).expect("expected file to read. maybe the file was deleted or moved?");
    println!("{}", result);
    return result;
}

#[tauri::command]
fn save_file(file_content: String, window: Window) {
    println!("successfully invoked");
    if window.get_window("main").is_some() {
        let url = window.get_window("main").unwrap().url();
        println!("{:?}", url);
        let file_name = url.query().unwrap().replace("file=","");
        println!("file name: {file_name}");
        let mut path  = Path::new("src/files/").join(format!("{}",file_name));

        println!("{:?}", file_content);

        fs::write(path, file_content).expect("expected content to add");
    } else {
        println!("main window could not be retrieved (is_none)");
        println!("{:#?}", window.clone())
    }
}
#[tauri::command]
fn delete_this_file(file_name: String) {
    let mut path  = Path::new("src/files/").join(format!("{}",file_name));
    remove_file(path).expect("expected file to delete");
}
