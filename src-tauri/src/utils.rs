use lazy_static::lazy_static;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

lazy_static! {
    pub static ref DEPARTEMENTS: HashMap<String, String> = [
        ("01", "Ain"),
        ("02", "Aisne"),
        ("03", "Allier"),
        ("04", "Alpes-de-Haute-Provence"),
        ("05", "Hautes-Alpes"),
        ("06", "Alpes-Maritimes"),
        ("07", "Ardèche"),
        ("08", "Ardennes"),
        ("09", "Ariège"),
        ("10", "Aube"),
        ("11", "Aude"),
        ("12", "Aveyron"),
        ("13", "Bouches-du-Rhône"),
        ("14", "Calvados"),
        ("15", "Cantal"),
        ("16", "Charente"),
        ("17", "Charente-Maritime"),
        ("18", "Cher"),
        ("19", "Corrèze"),
        ("21", "Côte-d'Or"),
        ("22", "Côtes-d'Armor"),
        ("23", "Creuse"),
        ("24", "Dordogne"),
        ("25", "Doubs"),
        ("26", "Drôme"),
        ("27", "Eure"),
        ("28", "Eure-et-Loir"),
        ("29", "Finistère"),
        ("2A", "Corse-du-Sud"),
        ("2B", "Haute-Corse"),
        ("30", "Gard"),
        ("31", "Haute-Garonne"),
        ("32", "Gers"),
        ("33", "Gironde"),
        ("34", "Hérault"),
        ("35", "Ille-et-Vilaine"),
        ("36", "Indre"),
        ("37", "Indre-et-Loire"),
        ("38", "Isère"),
        ("39", "Jura"),
        ("40", "Landes"),
        ("41", "Loir-et-Cher"),
        ("42", "Loire"),
        ("43", "Haute-Loire"),
        ("44", "Loire-Atlantique"),
        ("45", "Loiret"),
        ("46", "Lot"),
        ("47", "Lot-et-Garonne"),
        ("48", "Lozère"),
        ("49", "Maine-et-Loire"),
        ("50", "Manche"),
        ("51", "Marne"),
        ("52", "Haute-Marne"),
        ("53", "Mayenne"),
        ("54", "Meurthe-et-Moselle"),
        ("55", "Meuse"),
        ("56", "Morbihan"),
        ("57", "Moselle"),
        ("58 ", "Nièvre"),
        ("59", "Nord"),
        ("60", "Oise"),
        ("61", "Orne"),
        ("62", "Pas-de-Calais"),
        ("63", "Puy-de-Dôme"),
        ("64", "Pyrénées-Atlantiques"),
        ("65", "Hautes-Pyrénées"),
        ("66", "Pyrénées-Orientales"),
        ("67", "Bas-Rhin"),
        ("68", "Haut-Rhin"),
        ("69", "Rhône"),
        ("70", "Haute-Saône"),
        ("71", "Saône-et-Loire"),
        ("72", "Sarthe"),
        ("73", "Savoie"),
        ("74", "Haute-Savoie"),
        ("75", "Paris"),
        ("76", "Seine-Maritime"),
        ("77", "Seine-et-Marne"),
        ("78", "Yvelines"),
        ("79", "Deux-Sèvres"),
        ("80", "Somme"),
        ("81", "Tarn"),
        ("82", "Tarn-et-Garonne"),
        ("83", "Var"),
        ("84", "Vaucluse"),
        ("85", "Vendée"),
        ("86", "Vienne"),
        ("87", "Haute-Vienne"),
        ("88", "Vosges"),
        ("89", "Yonne"),
        ("90", "Territoire de Belfort"),
        ("91", "Essonne"),
        ("92", "Hauts-de-Seine"),
        ("93", "Seine-Saint-Denis"),
        ("94", "Val-de-Marne"),
        ("95", "Val-d'Oise"),
        ("971", "Guadeloupe"),
        ("972", "Martinique"),
        ("973", "Guyane"),
        ("974", "La Réunion"),
        ("976", "Mayotte"),
    ]
    .iter()
    .map(|&(code, name)| (code.to_string(), name.to_string()))
    .collect();
    pub static ref Vulcain_Colors: Vec<(u8, u8, u8)> = [
        (0, 0, 0),
        (4, 25, 30),
        (25, 50, 60),
        (50, 200, 80),
        (80, 200, 120),
    ]
    .to_vec();
    pub static ref Parasite_Colors: Vec<(u8, u8, u8)> =
        [(255, 255, 255), (128, 128, 128), (14, 14, 14)].to_vec();
}

pub fn get_departement_list() -> HashMap<String, String> {
    DEPARTEMENTS.clone()
}

pub fn get_departement_name(code: &str) -> Option<String> {
    DEPARTEMENTS.get(code).map(|name| name.to_string())
}

pub fn get_departement_code(name: &str) -> Option<String> {
    DEPARTEMENTS.iter().find_map(|(code, n)| {
        if n == name {
            Some(code.to_string())
        } else {
            None
        }
    })
}

pub fn get_departements_names() -> Vec<String> {
    DEPARTEMENTS.values().cloned().collect()
}

//////----------------file management-----------------//////

pub fn create_directory_if_not_exists(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn compress_folder(
    folder_directory_path: &str,
    folder_name: &str,
    destination_directory_path: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("7z");
    command.arg("a");

    let archive_path = format!("{}.zip", folder_name);

    command.arg(archive_path);
    command.current_dir(
        if let Some(destination_directory_path) = destination_directory_path {
            destination_directory_path
        } else {
            folder_directory_path
        },
    );
    command.arg(folder_name);
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!("Failed to execute command: {:?}", output).into());
    }

    Ok(())
}

pub fn extract_archive(
    archive_path: &str,
    destination_directory_path: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("7z");
    command.arg("x");

    if let Some(destination_directory_path) = destination_directory_path {
        command.arg(format!("-o{}", destination_directory_path));
    } else {
        let parent_dir = std::path::Path::new(archive_path).parent().unwrap();
        command.arg(format!("-o{}", parent_dir.to_str().unwrap()));
    }

    command.arg(archive_path);
    let output = command.output()?;
    if !output.status.success() {
        return Err(format!("Failed to execute command: {:?}", output).into());
    }

    Ok(())
}

pub fn find_filepath_in_archive(archive_path: &str, file_name: &str) -> Result<Option<String>, Box<dyn Error>> {
    let output = Command::new("7z")
        .args(&["l", archive_path])
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains(file_name) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(path) = parts.last() {
                    let path_str = path.to_string();
                    if let Some(pos) = path_str.rfind(file_name) {
                        return Ok(Some(path_str[..pos].to_string()));
                    }
                }
            }
        }
        Ok(None)
    } else {
        Err(format!("Failed to execute command: {:?}", output).into())
    }
}

pub fn extract_specific_file(
    archive_path: &str,
    file_name: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    create_directory_if_not_exists(output_dir)?;
    let output_path = Path::new(output_dir).join(file_name);

    let output = Command::new("7z")
        .args(&["e", archive_path, "-r", file_name])
        .arg(format!("-o{}", output_dir))
        .output()?;

    if output.status.success() {
        println!("File extracted to: {}", output_path.display());
    } else {
        println!("Failed to extract the file.");
        println!("7z stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("7z stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn extract_specific_folder(
    archive_path: &str,
    folder_name: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    create_directory_if_not_exists(output_dir)?;
    let output_path = Path::new(output_dir).join(folder_name);

    let output = Command::new("7z")
        .args(&["e", archive_path, "-r", folder_name])
        .arg(format!("-o{}", output_path.display()))
        .output()?;

    if output.status.success() {
        println!("Folder extracted to: {}", output_path.display());
    } else {
        println!("Failed to extract the folder.");
        println!("7z stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("7z stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}




