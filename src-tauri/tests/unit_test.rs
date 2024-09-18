use eve_mapmaker::web_request;
use web_request::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_departement_shp_file_success() {
        let result = get_departement_shp_file("2A");
        assert_eq!(result.unwrap(), "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z");
    }

    #[test]
    fn test_get_departement_shp_file_no_file_found() {
        let result = get_departement_shp_file("99");
        assert_eq!(result.unwrap(), "No file found");
    }

    #[tokio::test]
    async fn test_download_and_extract_shp_file_success() {
        let url = "https://data.geopf.fr/telechargement/download/BDFORET/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10/BDFORET_2-0__SHP_LAMB93_D02A_2017-05-10.7z";
        let _ = download_and_extract_shp_file(url, "2A").await;
        assert!(std::path::Path::new("resources/data_2A").exists());
    }
}
