use crate::utils::create_directory_if_not_exists;
use pyo3::{prelude::*, types::PyDict};

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

#[pyfunction]
pub fn setup_basic_veg_layer(project_name: &str, layer_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsCategorizedSymbolRenderer, QgsField, QgsFillSymbol, QgsRendererCategory
from qgis.PyQt.QtGui import QColor
project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0] 
essence_field_index = layer.fields().indexFromName('ESSENCE')
if essence_field_index == -1:
    raise Exception("'ESSENCE' field not found")
unique_values = layer.uniqueValues(essence_field_index)
categories = []
for value in unique_values:
    symbol = QgsFillSymbol.createSimple({{'color': ''50,200,80,255'', 'outline_style': 'no'}})
    category = QgsRendererCategory(value, symbol, str(value))
    categories.append(category)

renderer = QgsCategorizedSymbolRenderer('ESSENCE', categories)
layer.setRenderer(renderer)
layer.triggerRepaint()
project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Layer {} in project {} categorized by ESSENCE successfully",
            layer_name, project_name
        ))
    })
}

#[pyfunction]
pub fn get_layer_fields_by_category(
    project_name: &str,
    layer_name: &str,
    category: &str,
) -> PyResult<Vec<String>> {
    let code = format!(
        r#"
from qgis.core import QgsProject
project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0]
category_field_index = layer.fields().indexFromName("{category}")
if category_field_index == -1:
    raise Exception("Category field not found")
unique_values = list(layer.uniqueValues(category_field_index))
unique_values
"#,
        project_name = project_name,
        layer_name = layer_name,
        category = category
    );

    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let locals = PyDict::new_bound(py);
        py.run_bound(&code, None, Some(&locals))?;
        let result: Vec<String> = match locals.get_item("unique_values") {
            Ok(Some(values)) => values.extract()?,
            Ok(None) => Vec::new(),
            Err(_) => todo!(),
        };
        Ok(result)
    })
}

pub fn edit_layer_field_color(
    project_name: &str,
    layer_name: &str,
    category: &str,
    field_name: &str,
    color: &str
) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsCategorizedSymbolRenderer, QgsRendererCategory, QgsFillSymbol
from qgis.PyQt.QtGui import QColor
project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0]
category_field_index = layer.fields().indexFromName("{category}")
if category_field_index == -1:
    raise Exception("Category field not found")
unique_values = layer.uniqueValues(category_field_index)
categories = []
for value in unique_values:
    if str(value) == "{field_name}":
        symbol = QgsFillSymbol.createSimple({{'color': '{color}', 'outline_style': 'no'}})
    else:
        symbol = QgsFillSymbol.createSimple({{'color': '50,200,80,255', 'outline_style': 'no'}})
    category = QgsRendererCategory(value, symbol, str(value))
    categories.append(category)

renderer = QgsCategorizedSymbolRenderer('{category}', categories)
layer.setRenderer(renderer)
layer.triggerRepaint()
project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name,
        field_name = field_name,
        color = color,
        category = category
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Field color for {} in layer {} in project {} updated to {} for category {}",
            field_name, layer_name, project_name, color, category
        ))
    })
}
