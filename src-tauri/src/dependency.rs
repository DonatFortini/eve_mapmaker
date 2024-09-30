use std::process::Command;
use std::str;

#[derive(Debug)]
pub enum DependencyError {
    QgisNotInstalled,
    PythonNotInstalled,
    PythonModuleNotInstalled(String),
    SevenZipNotInstalled,
}

/// Check if all dependencies are installed.
///
/// # Returns
/// - Result<(), DependencyError>
pub fn check_dependencies() -> Result<(), DependencyError> {
    let qgis_command = if cfg!(target_os = "windows") {
        "qgis-bin.exe"
    } else if cfg!(target_os = "macos") {
        "qgis"
    } else {
        "qgis"
    };

    if Command::new(qgis_command)
        .arg("--version")
        .output()
        .is_err()
    {
        return Err(DependencyError::QgisNotInstalled);
    }

    let python_command = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };

    let python_output = Command::new(python_command).arg("--version").output();

    match python_output {
        Ok(output) => {
            let version_str = str::from_utf8(&output.stdout).unwrap_or("");
            println!("Python found: {}", version_str);
        }
        Err(_) => {
            return Err(DependencyError::PythonNotInstalled);
        }
    }

    let check_modules = vec!["qgis.core", "qgis.gui"];

    for module in check_modules {
        let output = Command::new(python_command)
            .arg("-c")
            .arg(format!("import {}", module))
            .output();

        if output.is_err() || !output.unwrap().status.success() {
            return Err(DependencyError::PythonModuleNotInstalled(
                module.to_string(),
            ));
        }
    }

    let seven_zip_command = if cfg!(target_os = "windows") {
        "7z.exe"
    } else {
        "7z"
    };

    if Command::new(seven_zip_command)
        .arg("--help")
        .output()
        .is_err()
    {
        return Err(DependencyError::SevenZipNotInstalled);
    }

    Ok(())
}
