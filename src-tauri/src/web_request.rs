use futures_util::StreamExt;
use reqwest;
use scraper::{Html, Selector};
use std::{error::Error, fs, path::Path};
use tokio::{fs::File, io::AsyncWriteExt};

/// Gets the URL of a SHP file from the IGN Database.
///
/// # Parameters
/// - `code`: A string slice that holds the department code.
/// - `url`: A string slice that holds the URL to a specific page of the IGN database.
///
/// # Returns
/// - A string slice representing the URL of the SHP file archive that corespond to the departement.
pub async fn get_departement_shp_file_url(code: &str, url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a")?;

    let mut shp_files: Vec<_> = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| href.contains(&format!("D0{}", code)))
        .collect();

    if shp_files.is_empty() {
        return Err("No file found".into());
    }

    shp_files.sort_by(|a, b| {
        let date_a = a.split('_').last().unwrap().split('.').next().unwrap();
        let date_b = b.split('_').last().unwrap().split('.').next().unwrap();
        date_b.cmp(date_a)
    });

    Ok(shp_files[0].to_string())
}

/// Downloads a SHP file from a given URL from the IGN Database.
///
/// - If the URL contains "BDTOPO", the name will be "BDTOPO".
/// - If the URL contains "BDFORET", the name will be "BDFORET".
/// - Otherwise, the name will be "unknown".
///
/// # Parameters
/// - `url`: A string slice that holds the URL to be checked.
/// - `code`: A string slice that holds the department code.
///
/// # Returns
/// - A string slice representing the determined name.
pub async fn download_shp_file(url: &str, code: &str) -> Result<(), Box<dyn Error>> {
    let name = match url {
        url if url.contains("BDTOPO") => "BDTOPO",
        url if url.contains("BDFORET") => "BDFORET",
        _ => "unknown",
    };
    let tmp_folder_path = "tmp";
    let archive_path = format!("{}/{}_{}.7z", tmp_folder_path, name, code);

    if Path::new(&archive_path).exists() {
        fs::remove_file(&archive_path)?;
    }

    let mut file = File::create(archive_path).await?;
    let mut stream = reqwest::get(url).await?.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    Ok(())
}
