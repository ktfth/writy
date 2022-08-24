#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu};

fn main() {
  let open = CustomMenuItem::new("open".to_string(), "Open");
  let save = CustomMenuItem::new("save".to_string(), "Save");
  let save_as = CustomMenuItem::new("save_as".to_string(), "Save as...");
  let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(save_as));
  let menu = Menu::new()
    .add_submenu(submenu);
  tauri::Builder::default()
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
