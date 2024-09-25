// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eve_mapmaker::qgis_api_wrapper::{create_blank_project, load_vector_layer_to_project};
use eve_mapmaker::utils::get_departement_list;
use eve_mapmaker::web_request::{download_shp_file, get_departement_shp_file_url};
use std::collections::HashMap;
use tauri::Manager;

#[tauri::command]
async fn open_new_project(
    app_handle: tauri::AppHandle,
    code: String,
    name: String,
) -> Result<(), String> {
    let url1 = get_departement_shp_file_url(
        &code,
        "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
    )
    .map_err(|e| format!("Error getting shp file url1: {:?}", e))?;

    let url2 =
        get_departement_shp_file_url(&code, "https://geoservices.ign.fr/bdforet#telechargementv2")
            .map_err(|e| format!("Error getting shp file url2: {:?}", e))?;

    // Notify frontend about step 1 completion
    app_handle
        .emit_all("progress-update", "Loading Map Data")
        .unwrap();

    download_shp_file(&url1, &code)
        .await
        .map_err(|e| format!("Error downloading shp file from url1: {:?}", e))?;
    download_shp_file(&url2, &code)
        .await
        .map_err(|e| format!("Error downloading shp file from url2: {:?}", e))?;

    // Notify frontend about step 2 completion
    app_handle
        .emit_all("progress-update", "Downloading Files")
        .unwrap();

    create_blank_project(&name).map_err(|e| format!("Error creating QGIS project: {:?}", e))?;

    // Notify frontend about step 3 completion
    app_handle
        .emit_all("progress-update", "Preparing Layers")
        .unwrap();

    load_vector_layer_to_project(&name, "layer_path1", "layer_name1")
        .map_err(|e| format!("Error loading layer to project: {:?}", e))?;
    load_vector_layer_to_project(&name, "layer_path2", "layer_name2")
        .map_err(|e| format!("Error loading second layer: {:?}", e))?;

    // Notify frontend about step 4 completion
    app_handle
        .emit_all("progress-update", "Finalizing")
        .unwrap();

    Ok(())
}

#[tauri::command]
fn get_dpts_list() -> HashMap<String, String> {
    return get_departement_list();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project, get_dpts_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // required to run python in a multi-threaded environment
    pyo3::prepare_freethreaded_python();
}
