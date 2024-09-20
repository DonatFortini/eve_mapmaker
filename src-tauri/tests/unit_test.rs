use eve_mapmaker::utils;
use eve_mapmaker::web_request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_departement_shp_file_success() {
        let result = web_request::get_departement_shp_file("2A");
        assert_eq!(result.unwrap(), "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z");
    }

    #[test]
    fn test_get_departement_shp_file_no_file_found() {
        let result = web_request::get_departement_shp_file("99");
        assert_eq!(result.unwrap(), "No file found");
    }

    #[tokio::test]
    async fn test_download_and_extract_shp_file_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z";
        let _ = web_request::download_and_extract_shp_file(url, "2A").await;
        assert!(std::path::Path::new("resources/data_2A").exists());
    }

    #[test]
    fn test_compression_successfull() {
        let folder_path = "resources/data_2A";
        let release_name = "data_2A";
        let destination_path = "resources";
        utils::compress_folder_for_release(folder_path, release_name, destination_path);
        assert!(std::path::Path::new("resources/data_2A.zip").exists());
    }
}
