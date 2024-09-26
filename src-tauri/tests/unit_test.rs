use eve_mapmaker::app_setup;
use eve_mapmaker::dependency;
use eve_mapmaker::qgis_api_wrapper;
use eve_mapmaker::utils;
use eve_mapmaker::web_request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_check_success() {
        let result = app_setup::setup_check();
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_dependencies_success() {
        let result = dependency::check_dependencies();
        assert!(result.is_ok());
    }

    #[test]
    fn test_compression_successfull() {
        let directory_path = "resources";
        let folder_name = "data_2A";
        utils::compress_folder(&directory_path, &folder_name, None).unwrap();
        assert!(std::path::Path::new("resources/data_2A.zip").exists());
    }

    #[test]
    fn test_compression_successfull_into_tmp() {
        let directory_path = "resources";
        let folder_name = "data_2A";
        utils::compress_folder(&directory_path, &folder_name, Some("tmp")).unwrap();
        assert!(std::path::Path::new("tmp/data_2A.zip").exists());
    }

    #[test]
    fn test_decompression_successfull() {
        let archive_path = "resources/data_2A.zip";
        let _ = utils::extract_archive(archive_path, None);
        assert!(std::path::Path::new("resources/data_2A").exists());
    }

    #[test]
    fn test_decompression_successfull_into_tmp() {
        let archive_path = "resources/data_2A.zip";
        let _ = utils::extract_archive(archive_path, Some("tmp"));
        assert!(std::path::Path::new("tmp/data_2A").exists());
    }

    #[test]
    fn test_extract_specific_file_success() {
        let archive_path = "tmp/BDFORET_2A.7z";
        let file_name = "FORMATION_VEGETALE.shp";
        let output_dir = "resources/QGIS/test";
        let _ = utils::extract_specific_file(archive_path, file_name, output_dir);
        assert!(std::path::Path::new("resources/QGIS/test/FORMATION_VEGETALE.shp").exists());
    }

    #[test]
    fn test_extract_specific_folder_success() {
        let archive_path = "tmp/BDFORET_2A.7z";
        let folder = utils::find_filepath_in_archive(archive_path, "FORMATION_VEGETALE.shp")
            .unwrap()
            .unwrap();
        let output_dir = "resources/QGIS/test";
        let _ =
            utils::extract_specific_folder(archive_path, &folder, output_dir, Some("Vegetation"));
        assert!(
            std::path::Path::new("resources/QGIS/test/Vegetation/FORMATION_VEGETALE.shp").exists()
        );
    }

    #[test]
    fn test_find_file_in_archive_success() {
        let archive_path = "tmp/BDFORET_2A.7z";
        let file_name = "FORMATION_VEGETALE.shp";
        let result = utils::find_filepath_in_archive(archive_path, file_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().unwrap(), "BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET/1_DONNEES_LIVRAISON/BDF_2-0_SHP_LAMB93_D02A/");
    }

    // test IGN

    #[test]
    fn test_get_departement_shp_forest_url_success() {
        let result = web_request::get_departement_shp_file_url(
            "2A",
            "https://geoservices.ign.fr/bdforet#telechargementv2",
        );
        assert_eq!(
            result.unwrap(),
            "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z"
        );
    }

    #[test]
    fn test_get_departement_shp_forest_no_file_found() {
        let result = web_request::get_departement_shp_file_url(
            "99",
            "https://geoservices.ign.fr/bdforet#telechargementv2",
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No file found");
    }

    #[test]
    fn test_get_departement_shp_topo_url_success() {
        let result = web_request::get_departement_shp_file_url(
            "2A",
            "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
        );
        assert_eq!(
            result.unwrap(),
            "https://data.geopf.fr/telechargement/download/BDTOPO/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15.7z"
        );
    }

    #[test]
    fn test_get_departement_shp_topo_no_file_found() {
        let result = web_request::get_departement_shp_file_url(
            "99",
            "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No file found");
    }

    #[tokio::test]
    async fn test_download_shp_file_foret_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z";
        match web_request::download_shp_file(url, "2A").await {
            Ok(_) => {
                assert!(std::path::Path::new("tmp/BDFORET_2A.7z").exists());
            }
            Err(e) => {
                panic!("Download failed: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_download_shp_file_topo_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDTOPO/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15.7z";
        match web_request::download_shp_file(url, "2A").await {
            Ok(_) => {
                assert!(std::path::Path::new("tmp/BDTOPO_2A.7z").exists());
            }
            Err(e) => {
                panic!("Download failed: {:?}", e);
            }
        }
    }

    // test qgis api wrapper
    #[test]
    fn test_qgis_api_create_blank_project_success() {
        // for test purpose, we need to prepare the python environment
        pyo3::prepare_freethreaded_python();
        let result = qgis_api_wrapper::create_blank_project("test");
        assert!(result.is_ok(), "Error: {:?}", result.err());
    }

    #[test]
    fn test_qgis_api_load_vector_layer_to_project_success() {
        // for test purpose, we need to prepare the python environment
        pyo3::prepare_freethreaded_python();
        let result = qgis_api_wrapper::load_vector_layer_to_project(
            "resources/QGIS/test/test.qgz",
            "resources/QGIS/test/Vegetation/FORMATION_VEGETALE.shp",
            "BDFORET_2A",
        );
        assert!(result.is_ok(), "Error: {:?}", result.err());
    }

    #[test]
    fn test_edit_veg_layer_success() {
        pyo3::prepare_freethreaded_python();
        let result = qgis_api_wrapper::edit_veg_layer("resources/QGIS/test/test.qgz", "BDFORET_2A");
        assert!(result.is_ok(), "Error: {:?}", result.err());
    }

    
}
