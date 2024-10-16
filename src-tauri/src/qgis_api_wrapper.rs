use crate::utils::{self, create_directory_if_not_exists};
use pyo3::{prelude::*, types::PyDict};

const QGIS_APP_PATHS: &[(&str, &str)] = &[
    ("windows", "AppData\\Roaming\\QGIS\\QGIS3"),
    ("linux", ".local/share/QGIS/QGIS3"),
    ("macos", "Library/Application Support/QGIS/QGIS3"),
];

fn get_qgis_app_path(os: &str) -> Option<&'static str> {
    QGIS_APP_PATHS
        .iter()
        .find_map(|&(key, path)| if key == os { Some(path) } else { None })
}

fn run_python_code(py: Python, code: &str) -> PyResult<()> {
    py.run_bound(code, None, None)?;
    Ok(())
}

#[pyfunction]
pub fn initialize_qgis_app_path() -> PyResult<String> {
    let os = utils::get_operating_system();
    let path = get_qgis_app_path(&os).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Unsupported operating system")
    })?;

    let code = format!(
        r#"
from qgis.core import QgsApplication
QgsApplication.setPrefixPath("{path}", True)
"#,
        path = path
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
        Ok("QGIS app path initialized".to_string())
    })
}

#[pyfunction]
pub fn create_blank_project(project_name: &str) -> PyResult<String> {
    let project_folder = format!("resources/QGIS/{}", project_name);
    let project_file_path = format!("{}/{}.qgz", project_folder, project_name);

    let _ = create_directory_if_not_exists(&project_folder);

    let code = format!(
        r#"
from qgis.core import QgsProject
project = QgsProject.instance()
project.clear()
project.write("{project_file_path}")
"#,
        project_file_path = project_file_path
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
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
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsVectorLayer
project = QgsProject.instance()
project.read("{project_name}")
layer = QgsVectorLayer("{layer_path}", "{layer_name}", "ogr")
if not layer.isValid():
    raise Exception("Layer not valid")
else:
    project.addMapLayer(layer)
project.write(project.fileName())
"#,
        project_name = project_name,
        layer_path = layer_path,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
        Ok(format!(
            "Layer {} loaded to project {}",
            layer_name, project_name
        ))
    })
}

#[pyfunction]
pub fn create_tree_group(project_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsLayerTreeGroup
project = QgsProject.instance()
project.read("{project_name}")
root = project.layerTreeRoot()
group = root.addGroup("combustion")
group.addGroup("BDTOPO")
group.addGroup("Vegetation")
project.write(project.fileName())
"#,
        project_name = project_name
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
        Ok(format!(
            "Group 'combustion' created in project {}",
            project_name
        ))
    })
}

#[pyfunction]
pub fn setup_basic_veg_layer(project_name: &str, layer_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsCategorizedSymbolRenderer, QgsField, QgsFillSymbol, QgsRendererCategory, QgsLayerTreeGroup
from qgis.PyQt.QtGui import QColor

project = QgsProject.instance()
project.read("{project_name}")

layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0]

root = project.layerTreeRoot()
combustion_group = root.findGroup('combustion')
vegetation_group = combustion_group.findGroup('Vegetation')

if not vegetation_group:
    raise Exception("Group 'Vegetation' not found")
layer_node = root.findLayer(layer.id())
if layer_node:
    parent = layer_node.parent()
    parent.removeChildNode(layer_node)

vegetation_group.addLayer(layer)
essence_field_index = layer.fields().indexFromName('ESSENCE')
if essence_field_index == -1:
    raise Exception("'ESSENCE' field not found")

unique_values = layer.uniqueValues(essence_field_index)

categories = []
for value in unique_values:
    if str(value) in ['Feuillus', 'Châtaignier', 'Chênes sempervirents', 'Chênes décidus', 'Hêtre']:
        symbol = QgsFillSymbol.createSimple({{'color': '80,200,120,255', 'outline_style': 'no'}})
    elif str(value) in ['NC', 'NR']:
        symbol = QgsFillSymbol.createSimple({{'color': '25,50,60,255', 'outline_style': 'no'}})
    else:
        symbol = QgsFillSymbol.createSimple({{'color': '50,200,80,255', 'outline_style': 'no'}})
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
        run_python_code(py, &code)?;
        Ok(format!(
            "Layer {} in project {} categorized by ESSENCE, moved to 'combustion' group, and processed successfully",
            layer_name, project_name
        ))
    })
}

#[pyfunction]
pub fn setup_basic_topo_layer(project_name: &str, layer_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsSimpleLineSymbolLayer, QgsSimpleFillSymbolLayer, QgsSymbol, QgsLayerTreeGroup
from qgis.PyQt.QtGui import QColor

project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception(f"Layer '{layer_name}' not found in the project '{project_name}'")
layer = layer[0]

root = project.layerTreeRoot()
combustion_group = root.findGroup('combustion')
bdtopo_group = combustion_group.findGroup('BDTOPO')

if not bdtopo_group:
    raise Exception("Group 'BDTOPO' not found")

layer_node = root.findLayer(layer.id())
if layer_node:
    current_parent = layer_node.parent()
    current_parent.removeChildNode(layer_node)
    bdtopo_group.addLayer(layer)
try:
    if layer.name() in ["COURS_D_EAU", "TRONCON_DE_ROUTE", "TRONCON_DE_VOIE_FERREE"]:
        symbol = QgsSymbol.defaultSymbol(layer.geometryType())
        symbol.deleteSymbolLayer(0)
        symbol_layer = QgsSimpleLineSymbolLayer.create({{'color': '0,0,0,255', 'width': '0,46000'}})
        if symbol_layer:
            symbol.appendSymbolLayer(symbol_layer)
        else:
            raise Exception("Failed to create line symbol layer")
    else:
        symbol = QgsSymbol.defaultSymbol(layer.geometryType())
        symbol.deleteSymbolLayer(0)
        symbol_layer = QgsSimpleFillSymbolLayer.create({{'color': '0,0,0,255', 'outline_style': 'no'}})
        if symbol_layer:
            symbol.appendSymbolLayer(symbol_layer)
        else:
            raise Exception("Failed to create fill symbol layer")
    layer.renderer().setSymbol(symbol)
    layer.triggerRepaint()
    print(f"Styling applied to the layer '{layer_name}'.")
except Exception as e:
    raise Exception(f"Error applying styling to layer '{layer_name}': {{str(e)}}")
project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name,
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
        Ok(format!(
            "Layer '{}' in project '{}' categorized and moved to group 'combustion/BDTOPO' successfully.",
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
            Ok(Some(value)) => value.extract().unwrap_or_else(|_| vec![]),
            Ok(None) => vec![],
            Err(_) => vec![],
        };
        Ok(result)
    })
}

#[pyfunction]
pub fn edit_layer_field_color(
    project_name: &str,
    layer_name: &str,
    category: &str,
    field_name: &str,
    color: &str,
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
        run_python_code(py, &code)?;
        Ok(format!(
            "Field color for {} in layer {} in project {} updated to {} for category {}",
            field_name, layer_name, project_name, color, category
        ))
    })
}

#[pyfunction]
pub fn export_map_to_jpg(
    project_name: &str,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    output_image_name: &str,
) -> PyResult<String> {
    let zoom = 25000.0;
    let dpi = 63.5;

    let project_file_path = format!("resources/QGIS/{}.qgz", project_name);
    let output_image_path = format!("resources/QGIS/{}.jpg", output_image_name);

    let code = format!(
        r#"
from qgis.core import QgsProject, QgsPrintLayout, QgsLayoutItemMap, QgsLayoutExporter, QgsCoordinateReferenceSystem, QgsRectangle
from qgis.PyQt.QtCore import QSize, QRectF
try:
    project = QgsProject.instance()
    project.read("{project_file_path}")
    layout = QgsPrintLayout(project)
    layout.initializeDefaults()
    map_item = QgsLayoutItemMap(layout)
    layout.addLayoutItem(map_item)
    map_rect = QgsRectangle({xmin}, {ymin}, {xmax}, {ymax})
    map_item.setExtent(map_rect)
    map_item.setCrs(QgsCoordinateReferenceSystem('EPSG:2154'))
    map_item.setScale({zoom})
    map_item.setFixedSize(QSize(400, 400))
    map_item.attemptMove(QRectF(5, 5, 200, 150))
    exporter = QgsLayoutExporter(layout)
    export_settings = QgsLayoutExporter.ImageExportSettings()
    export_settings.dpi = {dpi}
    result = exporter.exportToImage("{output_image_path}", export_settings)
    if result == QgsLayoutExporter.Success:
        print("Map exported successfully to " + "{output_image_path}")
    else:
        print("Failed to export map")
except Exception as e:
    print("An error occurred: " + str(e))
"#,
        project_file_path = project_file_path,
        xmin = xmin,
        ymin = ymin,
        xmax = xmax,
        ymax = ymax,
        dpi = dpi,
        zoom = zoom,
        output_image_path = output_image_path,
    );

    Python::with_gil(|py| {
        run_python_code(py, &code)?;
        Ok(format!(
            "Map exported successfully to {}",
            output_image_path
        ))
    })
}
