use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[tokio::main]
pub async fn get_departement_shp_file(code: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://geoservices.ign.fr/bdforet#telechargementv2";
    let body = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a")?;

    let mut shp_files: Vec<String> = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter(|href| href.contains(&format!("D0{}", code)))
        .map(|href| href.to_string())
        .collect();

    if shp_files.is_empty() {
        return Ok("No file found".to_string());
    }

    if shp_files.len() > 1 {
        shp_files.sort_by(|a, b| {
            let date_a = a.split('_').last().unwrap().split('.').next().unwrap();
            let date_b = b.split('_').last().unwrap().split('.').next().unwrap();
            date_b.cmp(date_a)
        });
    }

    Ok(shp_files[0].clone())
}


pub async fn download_and_extract_shp_file(url: &str, code: &str) -> Result<(), Box<dyn Error>> {
    let body = reqwest::get(url).await?.bytes().await?;
    let resources_path = "resources";
    let archive_path = format!("{}/data_{}.7z", resources_path, code);

    create_directory_if_not_exists(resources_path)?;
    fs::write(&archive_path, &body)?;
    extract_archive(&archive_path, resources_path)?;
    fs::remove_file(&archive_path)?;

    Ok(())
}

fn create_directory_if_not_exists(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}


//TODO : fix this function
fn extract_archive(archive_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    Command::new("7z")
        .arg("x")
        .arg(archive_path)
        .arg(format!("-o{}", output_path))
        .output()?;
    Ok(())
}


//TODO add func to fetch LOCUS DB for house and roads