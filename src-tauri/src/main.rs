// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eve_mapmaker::qgis_api_wrapper::{
    create_blank_project, load_vector_layer_to_project, setup_basic_topo_layer,
    setup_basic_veg_layer,
};
use eve_mapmaker::utils::{get_departement_list, layer_full_extraction};
use eve_mapmaker::web_request::{download_shp_file, get_departement_shp_file_url};
use std::collections::HashMap;
use tauri::Manager;

///---------------------------------------------------------tauri commands---------------------------------------------------------

//TODO : fix concurency issue with tokio
#[tauri::command]
async fn open_new_project(
    app_handle: tauri::AppHandle,
    code: String,
    name: String,
) -> Result<(), String> {
    pyo3::prepare_freethreaded_python();
    create_blank_project(&name).map_err(|e| format!("Error creating QGIS project: {:?}", e))?;

    println!("project created");

    app_handle
        .emit_all("progress-update", "Recherche des fichiers")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    let urls = get_shp_file_urls(&code)?;

    println!("urls: {:?}", urls);

    app_handle
        .emit_all("progress-update", "Téléchargement des données")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    download_shp_files(&urls, &code).await?;

    println!("files downloaded");

    app_handle
        .emit_all("progress-update", "Preparation des Couches")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    prepare_layers(&name, &code)?;

    println!("layers prepared");

    app_handle
        .emit_all("progress-update", "Finalisation")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    println!("done");

    Ok(())
}

#[tauri::command]
fn get_dpts_list() -> HashMap<String, String> {
    return get_departement_list();
}

///---------------------------------------------------------main---------------------------------------------------------

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_new_project, get_dpts_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

///---------------------------------------------------------functions---------------------------------------------------------

fn get_shp_file_urls(code: &str) -> Result<Vec<String>, String> {
    let url1 = get_departement_shp_file_url(
        code,
        "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
    )
    .map_err(|e| format!("Error getting shp file url1: {:?}", e))?;

    let url2 =
        get_departement_shp_file_url(code, "https://geoservices.ign.fr/bdforet#telechargementv2")
            .map_err(|e| format!("Error getting shp file url2: {:?}", e))?;

    Ok(vec![url1, url2])
}

async fn download_shp_files(urls: &[String], code: &str) -> Result<(), String> {
    for url in urls {
        download_shp_file(url, code)
            .await
            .map_err(|e| format!("Error downloading shp file from {}: {:?}", url, e))?;
    }
    Ok(())
}

fn prepare_layers(name: &str, code: &str) -> Result<(), String> {
    layer_full_extraction("BDFORET", code, "FORMATION_VEGETALE", "Vegetation", None)
        .map_err(|e| format!("Error extracting layer1: {:?}", e))?;

    load_vector_layer_to_project(
        name,
        &format!("/resources/QGIS/{}/Vegetation/FORMATION_VEGETALE.shp", name),
        "BDFORET",
    )
    .map_err(|e| format!("Error loading layer to project: {:?}", e))?;

    let _ = setup_basic_veg_layer(name, "BDFORET");

    let topo_layers = [
        "TERRAIN_DE_SPORT",
        "RESERVOIR",
        "CONSTRUCTION_SURFACIQUE",
        "BATIMENT",
        "PLAN_D_EAU",
        "COURS_D_EAU",
        "ZONE_D_HABITATION",
        "ZONE_D_ACTIVITE_OU_D_INTERET",
        "TRONCON_DE_ROUTE",
        "TRONCON_DE_VOIE_FERREE",
        "PISTE_D_AERODROME",
        "ITINERAIRE_AUTRE",
        "EQUIPEMENT_DE_TRANSPORT",
        "AERODROME",
    ];

    for layer in topo_layers.iter() {
        layer_full_extraction("BDTOPO", code, layer, "Topographie", Some(layer))
            .map_err(|e| format!("Error extracting layer1: {:?}", e))?;

        load_vector_layer_to_project(
            name,
            &format!("/resources/QGIS/{}/Topographie/{}.shp", name, layer),
            layer,
        )
        .map_err(|e| format!("Error loading layer to project: {:?}", e))?;

        let _ = setup_basic_topo_layer(name, layer);
    }

    Ok(())
}
