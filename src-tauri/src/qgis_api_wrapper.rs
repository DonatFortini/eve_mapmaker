use crate::utils::create_directory_if_not_exists;
use pyo3::prelude::*;

#[pyfunction]
pub fn create_blank_project(project_name: &str) -> PyResult<String> {
    let project_folder = format!("resources/QGIS/{}", project_name);
    let project_file_path = format!("{}/{}.qgz", project_folder, project_name);

    let _ = create_directory_if_not_exists(&project_folder);

    let code = format!(
        r#"
from qgis.core import QgsProject
project = QgsProject.instance()
project.clear()  # Start with a blank project
project.write("{project_file_path}")
"#,
        project_file_path = project_file_path
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Project {} created at {}",
            project_name, project_folder
        ))
    })
}

#[pyfunction]
pub fn load_vector_layer_to_project(
    project_name: &str,
    layer_path: &str,
    layer_name: &str,
) -> PyResult<String> {
    let project_file_path = if project_name.ends_with(".qgz") {
        project_name.to_string()
    } else {
        format!("{}.qgz", project_name)
    };

    let code = format!(
        r#"
from qgis.core import QgsProject, QgsVectorLayer
project = QgsProject.instance()
project.read("{project_file_path}")
layer = QgsVectorLayer("{layer_path}", "{layer_name}", "ogr")
if not layer.isValid():
    raise Exception("Layer failed to load")
project.addMapLayer(layer)
project.write("{project_file_path}")
"#,
        project_file_path = project_file_path,
        layer_path = layer_path,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Layer {} loaded to project {}",
            layer_name, project_name
        ))
    })
}


///TODO finir categorisation
#[pyfunction]
pub fn edit_veg_layer(project_name: &str, layer_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsFillSymbol, QgsCategorizedSymbolRenderer
from qgis.PyQt.QtGui import QColor
project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0] 
renderer = layer.renderer()
symbol = QgsFillSymbol.createSimple({{'line_style' : 'no', 'color': '255,0,0,255'}})
renderer.setSymbol(symbol)

layer.triggerRepaint()

project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Layer {} in project {} edited successfully",
            layer_name, project_name
        ))
    })
}
