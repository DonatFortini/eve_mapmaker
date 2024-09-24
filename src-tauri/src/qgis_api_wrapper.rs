use pyo3::prelude::*;
use crate::utils::create_directory_if_not_exists;

#[pyfunction]
pub fn create_blank_project(project_name: &str) -> PyResult<String> {

    let project_folder = format!("resources/QGIS/{}", project_name);
    let project_file_path = format!("{}/{}.qgz", project_folder, project_name);
    
    let _ = create_directory_if_not_exists(&project_folder);

    let code = format!(
        r#"
from qgis.core import QgsProject

# Create a new project
project = QgsProject.instance()
project.clear()  # Start with a blank project

# Write the project to the specified file
project.write("{project_file_path}")
"#,
        project_file_path = project_file_path
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!("Project {} created at {}", project_name, project_folder))
    })
}


#[pyfunction]
pub fn load_vector_layer_to_project(
    project_name: &str,
    layer_path: &str,
    layer_name: &str,
) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsVectorLayer

# Load the project
project = QgsProject.instance()
project.read("{project_name}.qgz")

# Load the vector layer
layer = QgsVectorLayer("{layer_path}", "{layer_name}", "ogr")
if not layer.isValid():
    raise Exception("Layer failed to load")

# Add the layer to the project
project.addMapLayer(layer)

# Save the project after adding the layer
project.write("{project_name}.qgz")
"#,
        project_name = project_name,
        layer_path = layer_path,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!("Layer {} loaded to project {}", layer_name, project_name))
    })
}
