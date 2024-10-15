// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eve_mapmaker::app_setup::setup_check;
use eve_mapmaker::utils::{get_departement_list, get_previous_projects, layer_full_extraction};
use eve_mapmaker::web_request::{download_shp_file, get_departement_shp_file_url};
use eve_mapmaker::{qgis_api_wrapper::*, utils};
use std::collections::HashMap;
use std::path::Path;
use tauri::Manager;

//---------------------------------------------------------tauri commands---------------------------------------------------------

//TODO : CHANGE THE STEPS ORDER IN THE FRONTEND

#[tauri::command]
/// Create a new project with the given code and name.
///
/// # Parameters
/// - `code`: A string slice that holds the code of the department.
/// - `name`: A string slice that holds the name of the project.
///
/// # Returns
/// - Result<(), String>
async fn open_new_project(
    app_handle: tauri::AppHandle,
    code: String,
    name: String,
) -> Result<(), String> {
    app_handle
        .emit_all("progress-update", "Recherche des fichiers")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    let urls = get_shp_file_urls(&code).await?;

    println!("urls: {:?}", urls);

    app_handle
        .emit_all("progress-update", "Téléchargement des données")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    download_shp_files(&urls, &code).await?;

    println!("files downloaded");

    app_handle
        .emit_all("progress-update", "Initialisation du projet")
        .map_err(|e| format!("Error emitting progress update: {:?}", e))?;

    pyo3::prepare_freethreaded_python();

    initialize_qgis_app_path().map_err(|e| format!("Error initializing QGIS app path: {:?}", e))?;

    create_blank_project(&name).map_err(|e| format!("Error creating QGIS project: {:?}", e))?;

    println!("project created");

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
/// Get the list of departments.
///
/// # Returns
/// - HashMap<String, String> : A hashmap containing the code and name of the departments.
fn get_dpts_list() -> HashMap<String, String> {
    return get_departement_list();
}

#[tauri::command]
fn get_projects() -> HashMap<String, Vec<String>> {
    match get_previous_projects() {
        Ok(projects) => projects,
        Err(_) => HashMap::new(),
    }
}

#[tauri::command]
fn get_os() -> String {
    return utils::get_operating_system().to_string();
}

//---------------------------------------------------------main---------------------------------------------------------

fn main() {
    setup_check().expect("Setup check failed");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_new_project,
            get_dpts_list,
            get_projects,
            get_os
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//---------------------------------------------------------functions---------------------------------------------------------

/// Get the urls of the shp files for the given department code.
/// # Parameters
/// - `code`: A string slice that holds the code of the department.
/// # Returns
/// - Result<Vec<String>, String> : A vector containing the urls of the shp files.
async fn get_shp_file_urls(code: &str) -> Result<Vec<String>, String> {
    let url1 = get_departement_shp_file_url(
        code,
        "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
    )
    .await
    .map_err(|e| format!("Error getting shp file url1: {:?}", e))?;

    let url2 =
        get_departement_shp_file_url(code, "https://geoservices.ign.fr/bdforet#telechargementv2")
            .await
            .map_err(|e| format!("Error getting shp file url2: {:?}", e))?;

    Ok(vec![url1, url2])
}

/// Download the shp file from the given url.
/// # Parameters
/// - `url`: A string slice that holds the url of the shp file.
/// - `code`: A string slice that holds the code of the department.
/// # Returns
/// - Result<(), String> : An empty result or an error message.
async fn download_shp_files(urls: &[String], code: &str) -> Result<(), String> {
    println!("downloading shp files");
    if Path::new(format!("tmp/BDTOPO_{}.7z", code).as_str()).exists()
        && Path::new(format!("tmp/BDFORET_{}.7z", code).as_str()).exists()
    {
        return Ok(());
    }

    for url in urls {
        download_shp_file(url, code)
            .await
            .map_err(|e| format!("Error downloading shp file from {}: {:?}", url, e))?;
        println!("downloaded shp file from {}", url);
    }
    println!("done downloading shp files");
    Ok(())
}

//TODO : FIX THIS
/// Prepare the layers for the given project.
/// # Parameters
/// - `name`: A string slice that holds the name of the project.
/// - `code`: A string slice that holds the code of the department.
/// # Returns
/// - Result<(), String> : An empty result or an error message.
fn prepare_layers(name: &str, code: &str) -> Result<(), String> {
    println!("preparing layers");
    println!("name: {}", name);

    layer_full_extraction(
        "BDFORET",
        code,
        "FORMATION_VEGETALE",
        &format!("{}/Vegetation", name),
        None,
    )
    .map_err(|e| format!("Error extracting layer1: {:?}", e))?;

    println!("layer1 extracted");

    load_vector_layer_to_project(
        &format!("resources/QGIS/{}/{}.qgz", name, name),
        &format!(
            "resources/QGIS/{}/Vegetation/FORMATION_VEGETALE/FORMATION_VEGETALE.shp",
            name
        ),
        "BDFORET",
    )
    .map_err(|e| format!("Error loading layer to project: {:?}", e))?;

    println!("layer1 loaded");

    let _ = setup_basic_veg_layer(&format!("resources/QGIS/{}/{}.qgz", name, name), "BDFORET");

    println!("veg layer setup");

    let topo_layers = [
        "TERRAIN_DE_SPORT",
        "RESERVOIR",
        "CONSTRUCTION_SURFACIQUE",
        "BATIMENT",
        "PLAN_D_EAU",
        "COURS_D_EAU",
        "ZONE_D_HABITATION",
        "TRONCON_DE_ROUTE",
        "TRONCON_DE_VOIE_FERREE",
        "PISTE_D_AERODROME",
        "ZONE_D_ESTRAN",
        "EQUIPEMENT_DE_TRANSPORT",
        "AERODROME",
    ];

    for layer in topo_layers.iter() {
        layer_full_extraction(
            "BDTOPO",
            code,
            layer,
            &format!("{}/Topographie", name),
            Some(layer),
        )
        .map_err(|e| format!("Error extracting layer1: {:?}", e))?;

        println!("layer {} extracted", layer);

        load_vector_layer_to_project(
            &format!("resources/QGIS/{}/{}.qgz", name, name),
            &format!(
                "resources/QGIS/{}/Topographie/{}/{}.shp",
                name, layer, layer
            ),
            layer,
        )
        .map_err(|e| format!("Error loading layer to project: {:?}", e))?;

        println!("layer {} loaded", layer);

        let _ = setup_basic_topo_layer(
            &format!("resources/QGIS/{}/{}.qgz", name, name),
            layer,
            "BDTOPO",
        );

        println!("topo layer {} setup", layer);
    }

    Ok(())
}
