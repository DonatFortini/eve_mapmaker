use crate::utils::create_directory_if_not_exists;
use pyo3::{prelude::*, types::PyDict};

#[pyfunction]
/// Use the QGIS API to create a blank project.
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
///
/// # Returns
/// - py Error or Result.
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
/// Use the QGIS API to load a vector layer to a project.
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
/// - `layer_path`: A string slice that holds the path to the layer.
/// - `layer_name`: A string slice that holds the name of the layer.
///
/// # Returns
/// - py Error or Result.
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
/// Use the QGIS API to apply the basic setup needed for a vegetation layer.
/// (Categorize by ESSENCE, move to the bottom and apply a green color)
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
/// - `layer_name`: A string slice that holds the name of the layer.
///
/// # Returns
/// - py Error or Result.
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
    symbol = QgsFillSymbol.createSimple({{'color': '50,200,80,255', 'outline_style': 'no'}})
    category = QgsRendererCategory(value, symbol, str(value))
    categories.append(category)

renderer = QgsCategorizedSymbolRenderer('ESSENCE', categories)
layer.setRenderer(renderer)
layer.triggerRepaint()

layers = project.layerTreeRoot().children()
layer_node = project.layerTreeRoot().findLayer(layer.id())
project.layerTreeRoot().insertChildNode(len(layers), layer_node.clone())

project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Layer {} in project {} categorized by ESSENCE and moved to the bottom successfully",
            layer_name, project_name
        ))
    })
}

#[pyfunction]
/// Use the QGIS API to apply the basic setup needed for a topography layer.
/// (Categorize by layer name, set a black color and a width of 0.26 for lines and no outline for polygons)
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
/// - `layer_name`: A string slice that holds the name of the layer.
///
/// # Returns
/// - py Error or Result.
pub fn setup_basic_topo_layer(project_name: &str, layer_name: &str) -> PyResult<String> {
    let code = format!(
        r#"
from qgis.core import QgsProject, QgsSimpleLineSymbolLayer, QgsSimpleFillSymbolLayer, QgsSymbol
from qgis.PyQt.QtGui import QColor

project = QgsProject.instance()
project.read("{project_name}")
layer = project.mapLayersByName("{layer_name}")
if not layer:
    raise Exception("Layer not found")
layer = layer[0]

if layer_name in ["COURS_D_EAU", "TRONCON_DE_ROUTE", "TRONCON_DE_VOIE_FEREE", "ITINERAIRE_AUTRE"]:
    symbol = QgsSymbol.defaultSymbol(layer.geometryType())
    symbol.deleteSymbolLayer(0)
    symbol_layer = QgsSimpleLineSymbolLayer()
    symbol_layer.setColor(QColor(0, 0, 0))
    symbol_layer.setWidth(0.26)
    symbol.appendSymbolLayer(symbol_layer)
else:
    symbol = QgsSymbol.defaultSymbol(layer.geometryType())
    symbol.deleteSymbolLayer(0)
    symbol_layer = QgsSimpleFillSymbolLayer()
    symbol_layer.setColor(QColor(0, 0, 0))
    symbol.appendSymbolLayer(symbol_layer)

layer.renderer().setSymbol(symbol)
layer.triggerRepaint()
project.write(project.fileName())
"#,
        project_name = project_name,
        layer_name = layer_name
    );

    Python::with_gil(|py| {
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Layer {} in project {} categorized successfully",
            layer_name, project_name
        ))
    })
}

#[pyfunction]
/// Use the QGIS API to extract all the unique values of a field in a layer.
/// The field must be of type string.
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
/// - `layer_name`: A string slice that holds the name of the layer.
/// - `category`: A string slice that holds the name of the field.
///
/// # Returns
/// - A vector of strings representing the unique values of the field.
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

#[pyfunction]
/// Use the QGIS API to edit the color of a field in a layer.
/// The field must be of type string.
/// The color must be in the format "R,G,B,A" where R, G, B and A are integers between 0 and 255.
///
/// # Parameters
/// - `project_name`: A string slice that holds the name of the project.
/// - `layer_name`: A string slice that holds the name of the layer.
/// - `category`: A string slice that holds the name of the field.
/// - `field_name`: A string slice that holds the name of the field.
/// - `color`: A string slice that holds the color to be applied.
///
/// # Returns
/// - A string slice representing the result of the operation.
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
        py.run_bound(&code, None, None)?;
        Ok(format!(
            "Field color for {} in layer {} in project {} updated to {} for category {}",
            field_name, layer_name, project_name, color, category
        ))
    })
}

#[pyfunction]
/// Export a map from a QGIS project to a JPEG file with specified coordinates and output file name.
/// Zoom is fixed to 1:25000 and DPI is fixed to 63.5.
///
/// # Parameters
/// - `project_name`: The name of the QGIS project.
/// - `xmin`: The minimum x-coordinate of the bounding box.
/// - `ymin`: The minimum y-coordinate of the bounding box.
/// - `xmax`: The maximum x-coordinate of the bounding box.
/// - `ymax`: The maximum y-coordinate of the bounding box.
/// - `output_image_name`: The name of the output image file (without extension).
///
/// # Returns
/// - A `PyResult<String>` indicating success or failure.
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
from qgis.core import QgsProject, QgsLayout, QgsLayoutItemMap, QgsLayoutExporter, QgsCoordinateReferenceSystem, QgsRectangle
from qgis.PyQt.QtCore import QSize, QRectF
project = QgsProject.instance()
project.read("{project_file_path}")
layout = QgsLayout(project)
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
        py.run_bound(&code, None, None)?;
        let result = format!("Map exported successfully to {}", output_image_path);
        Ok(result)
    })
}
