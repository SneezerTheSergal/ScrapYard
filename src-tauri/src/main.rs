// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, Window};
use std::fs::*;
use std::io::{Error, Write};
use std::path::{PathBuf};
use log::error;
use tauri::GlobalShortcutManager;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default().setup(|app| {
        if let Some(mut path) = app.path_resolver().app_local_data_dir() {
            path.push("files");
            if path.exists() {
                println!("files folder should exist {:?}", path);
            } else {
                create_dir(path)?;
            }

        };
        app.path_resolver().app_local_data_dir().ok_or("could not resolve path to app_local_data_dir")?;
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![create_file, get_files, get_file_content, save_file, delete_this_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

type MyResult<T> = Result<T, ThisError>;

#[derive(Debug, thiserror::Error)]
enum ThisError {
    #[error("path not found")]
    PathNotFound,
    #[error("could not read")]
    CouldNotRead,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for ThisError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}


#[tauri::command]
fn create_file(app_handle: AppHandle,file_name: String) -> MyResult<()> {
    let mut corrected_file_name = file_name.replace(" ", "_");
    println!("entered create file with name: {corrected_file_name}");
    let path  = app_handle.path_resolver().app_local_data_dir();
    let fixed_path = match path {
        Some(path_unwrap) => {
            path_unwrap.join("files").join(format!("{}.txt",corrected_file_name))
        },
        _ => {
            println!("invalid path to app_local_data_dir");
            PathBuf::new()
        }
    };

    println!("entered create file with path: {:?}", fixed_path);
    println!("entered create file with full path: {:?}", fixed_path);
    File::create(fixed_path.clone())?;
    Ok(())


    /*if let new_file = File::create(fixed_path.clone()){
        match new_file {
            Ok(file) => {
                Ok(file)
            },
            Err(error) => {
                Err(error)
            }
        }
    };*/

}

#[tauri::command]
async fn get_files(app_handle: AppHandle) -> MyResult<Vec<String>> {
    if let Some(mut data_dir) = app_handle.path_resolver().app_local_data_dir() {
        data_dir.push("files");
        println!("{:?}", data_dir);
        Ok(read_dir(&data_dir)?.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().map(String::from)
            })
        }).collect())
    } else {
        Err(ThisError::PathNotFound)
    }
}


#[tauri::command]
fn get_file_content(app_handle: AppHandle ,file_name: String) -> MyResult<String> {
   let path  = app_handle.path_resolver().app_local_data_dir();
    let fixed_path = match path {
        Some(mut path_unwrap) => {
            path_unwrap.push("files");
            path_unwrap.push(file_name);
            path_unwrap
        },
        _ => {
            println!("invalid path to app_local_data_dir");
            PathBuf::new()
        }
    };
    println!("{:?}", fixed_path);
    let result = read_to_string(fixed_path).expect("expected content to load");
    println!("{}", result);
    return Ok(result);


}

#[tauri::command]
fn save_file(app_handle: AppHandle ,file_content: String, window: Window) {
    println!("successfully invoked");
    if window.get_window("main").is_some() {
        let url = window.get_window("main").unwrap().url();
        println!("{:?}", url);
        let file_name = url.query().unwrap().replace("file=","");
        println!("file name: {file_name}");
        let path  = app_handle.path_resolver().app_local_data_dir();
        let fixed_path = match path {
            Some(mut path_unwrap) => {
                path_unwrap.push("files");
                path_unwrap.push(file_name);
                path_unwrap
            },
            _ => {
                println!("invalid path to app_local_data_dir");
                PathBuf::new()
            }
        };



        write(fixed_path, file_content).expect("expected content to add");
    } else {
        println!("main window could not be retrieved (is_none)");
        println!("{:#?}", window.clone())
    }
}
#[tauri::command]
fn delete_this_file(app_handle: AppHandle,file_name: String) {
    let path  = app_handle.path_resolver().app_local_data_dir();
    let fixed_path = match path {
        Some(mut path_unwrap) => {
            path_unwrap.push("files");
            path_unwrap.push(file_name);
            path_unwrap
        },
        _ => {
            println!("invalid path to app_local_data_dir");
            PathBuf::new()
        }
    };
    remove_file(fixed_path).expect("expected file to delete");
}
