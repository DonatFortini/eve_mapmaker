use crate::dependency::{check_dependencies, DependencyError};
use crate::utils::create_directory_if_not_exists;
use std::fmt;

/// Check if all dependencies are installed.
///
/// # Returns
/// - Result<(), DependencyError>
pub fn setup_check() -> Result<(), String> {
    create_directory_if_not_exists("resources/QGIS").map_err(|e| e.to_string())?;
    create_directory_if_not_exists("tmp").map_err(|e| e.to_string())?;
    check_dependencies().map_err(|e| e.to_string())?;
    Ok(())
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyError::QgisNotInstalled => write!(f, "QGIS is not installed"),
            DependencyError::PythonNotInstalled => write!(f, "Python is not installed"),
            DependencyError::PythonModuleNotInstalled(module) => {
                write!(f, "Python module {} is not installed", module)
            }
            DependencyError::SevenZipNotInstalled => todo!(),
        }
    }
}
