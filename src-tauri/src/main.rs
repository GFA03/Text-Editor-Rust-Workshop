// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Serialize)]
pub enum EditorError{
    FileNotFound,
    IOError,
    OtherError
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Editor Error")
    }
}

#[tauri::command]
fn save_file(file_name: String, contents: String) -> Result<(), EditorError> {

    // TODO: Open the selected file and write the contents to it. Hint: use File::create()
    // OpenOptions nu suprascrie, ci doar da append
    
    let file = File::create(file_name);
    match file {
        Ok(mut file) => {
            match file.write_all(contents.as_bytes()){
                Ok(_) => Ok(()),
                Err(_) => Err(EditorError::IOError),
            }
        },
        Err(_) => Err(EditorError::IOError),
    }
}

#[tauri::command]
fn save_file_as(file_name: String, new_file_name: String) -> Result<(), EditorError> {
    match std::fs::rename(file_name, new_file_name){
        Ok(_) => Ok(()),
        Err(_) => Err(EditorError::IOError),
    }
    
}


#[tauri::command]
 fn create_new_file(file_name: String) -> Result< String, EditorError> {
    // if std::path::Path::new(&file_name.clone()).exists(){
    //     return Err(EditorError::FileNotFound);
    // } else {
    //     let file = File::create(file_name.clone());
    //     match file {
    //         Ok(_) => Ok(file_name),
    //         Err(_) => Err(EditorError::IOError),
    //     }
    // }
    if std::path::Path::new(&file_name.clone()).exists(){
        return Err(EditorError::FileNotFound);
    } else {
        match save_file(file_name.clone(), String::from("")) {
            Ok(_) => Ok(file_name),
            Err(_) => Err(EditorError::IOError),
        }
    }
    // TODO: Create the file and return its path
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_file, create_new_file, save_file_as]) // TODO: Register the save_file command
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
