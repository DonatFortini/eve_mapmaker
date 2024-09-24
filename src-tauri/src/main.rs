// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eve_mapmaker::utils::get_departements_names;

#[tauri::command]
async fn open_new_project(){
    
}

#[tauri::command]
async fn get_dpts_names() -> Vec<String> {
    return get_departements_names();
}





fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project,get_dpts_names])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // required to run python in a multi-threaded environment
    pyo3::prepare_freethreaded_python();
}
