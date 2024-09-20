use eve_mapmaker::utils;
use eve_mapmaker::web_request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_departement_shp_file_success() {
        let result = web_request::get_departement_geofile(
            "2A",
            "https://geoservices.ign.fr/bdforet#telechargementv2",
        );
        assert_eq!(result.unwrap(), "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z");
    }

    #[test]
    fn test_get_departement_shp_file_no_file_found() {
        let result = web_request::get_departement_geofile(
            "99",
            "https://geoservices.ign.fr/bdforet#telechargementv2",
        );
        assert_eq!(result.unwrap(), "No file found");
    }

    #[test]
    fn test_get_departement_shp_geo_success() {
        let result = web_request::get_departement_geofile(
            "2A",
            "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
        );
        assert_eq!(result.unwrap(), "https://data.geopf.fr/telechargement/download/BDTOPO/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15.7z");
    }

    #[test]
    fn test_get_departement_shp_geo_no_file_found() {
        let result = web_request::get_departement_geofile(
            "99",
            "https://geoservices.ign.fr/bdtopo#telechargementgpkgreg",
        );
        assert_eq!(result.unwrap(), "No file found");
    }

    #[tokio::test]
    async fn test_download_and_extract_shp_file_foret_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z";
        let _ = web_request::download_and_extract_shp_file(url, "2A").await;
        assert!(std::path::Path::new("resources/BDFORET_2A").exists());
    }

    #[tokio::test]
    async fn test_download_and_extract_shp_file_topo_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDTOPO/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15/BDTOPO_3-4_TOUSTHEMES_SHP_LAMB93_D02A_2024-06-15.7z";
        let _ = web_request::download_and_extract_shp_file(url, "2A").await;
        assert!(std::path::Path::new("resources/BDTOPO_2A").exists());
    }

    #[test]
    fn test_compression_successfull() {
        let folder_path = "resources/data_2A";
        let release_name = "data_2A";
        let destination_path = "resources";
        utils::compress_folder_for_release(folder_path, release_name, destination_path);
        assert!(std::path::Path::new("resources/data_2A.zip").exists());
    }

    #[test]
    fn test_decompression_successfull() {
        let archive_path = "resources/data_2A.zip";
        let _=utils::extract_archive(archive_path);
        assert!(std::path::Path::new("resources/data_2A").exists());
    }
}
