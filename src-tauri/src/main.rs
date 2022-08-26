#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use tauri::{CustomMenuItem, Menu, Submenu};
// use tauri::api::dialog::FileDialogBuilder;

fn main() {
  // let open = CustomMenuItem::new("open".to_string(), "Open");
  // let save = CustomMenuItem::new("save".to_string(), "Save");
  // let save_as = CustomMenuItem::new("save_as".to_string(), "Save as...");
  // let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(save_as));
  // let menu = Menu::new()
  //   .add_submenu(submenu);
  tauri::Builder::default()
    // .menu(menu)
    // .on_menu_event(|event| {
    //   match event.menu_item_id() {
    //     "open" => {
    //       println!("Open");
    //       FileDialogBuilder::new().add_filter("javascript", &["js"]).pick_file(move |file_path| {
    //         println!("Open: {:?}", file_path);
    //         let _out = event.window().emit("open-file-path", file_path.unwrap());
    //       });
    //     },
    //     "save" => {
    //       println!("Save");
    //       FileDialogBuilder::new().add_filter("javascript", &["js"]).save_file(move |file_path| {
    //         println!("Save: {:?}", file_path);
    //         // save_contents(file_path, file_value).unwrap();
    //         let _out = event.window().emit("save-file-path", file_path.unwrap());
    //       });
    //     },
    //     "save_as" => {
    //       println!("Save as...");
    //       FileDialogBuilder::new().add_filter("javascript", &["js"]).save_file(move |file_path| {
    //         println!("Save as: {:?}", file_path);
    //         let _out = event.window().emit("save-as-file-path", file_path.unwrap());
    //       });
    //     },
    //     _ => {},
    //   }
    // })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
