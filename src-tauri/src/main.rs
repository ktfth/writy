#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Manager, State};

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use std::sync::{Mutex};

// #[derive(Clone, serde::Serialize)]
// struct File {
//   value: string,
// }

fn save_contents(file_path: Option<PathBuf>, value: String) -> std::io::Result<()> {
  let mut file = File::open(file_path.unwrap().as_path())?;
  file.write_all(value.as_bytes())?;
  Ok(())
}

struct FileValue(Mutex<String>);

#[tauri::command]
fn set_file_value(value: String, file_value: State<'_, FileValue>) {
  *file_value.0.lock().unwrap() = value;
}

fn main() {
  let open = CustomMenuItem::new("open".to_string(), "Open");
  let save = CustomMenuItem::new("save".to_string(), "Save");
  let save_as = CustomMenuItem::new("save_as".to_string(), "Save as...");
  let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(save_as));
  let menu = Menu::new()
    .add_submenu(submenu);
  tauri::Builder::default()
    .manage(FileValue(Mutex::new(String::new())))
    .invoke_handler(tauri::generate_handler![set_file_value])
    .setup(|app| {
      let _id = app.listen_global("value-change", |event| {
        println!("{:?}", event.payload());
        // set_file_value(event.payload().unwrap().to_string(), app.state::<FileValue>());
      });

      Ok(())
    })
    .menu(menu)
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "open" => {
          println!("Open");
          FileDialogBuilder::new().add_filter("javascript", &["js"]).pick_file(|file_path| {
            println!("Open: {:?}", file_path);
          });
        },
        "save" => {
          println!("Save");
          FileDialogBuilder::new().add_filter("javascript", &["js"]).save_file(|file_path| {
            println!("Save: {:?}", file_path);
            // save_contents(file_path, file_value).unwrap();
          });
        },
        "save_as" => {
          println!("Save as...");
          FileDialogBuilder::new().add_filter("javascript", &["js"]).save_file(|file_path| {
            println!("Save as: {:?}", file_path);
          });
        },
        _ => {},
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
