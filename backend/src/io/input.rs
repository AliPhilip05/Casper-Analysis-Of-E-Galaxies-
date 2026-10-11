use crate::model::Galaxy;
use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;

/// Read galaxy measurements from a CSV with the required catalog columns.
/// Rows with values that cannot be parsed as numbers are skipped.
pub fn read_galaxies(path: impl AsRef<Path>) -> Result<Vec<Galaxy>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().flexible(true).from_path(path)?;
    let headers = reader.headers()?.clone();

    let column_index = |name: &str| -> Result<usize, Box<dyn Error>> {
        headers
            .iter()
            .position(|header| header == name)
            .ok_or_else(|| format!("Missing required CSV column: {name}").into())
    };

    let object_id_index = column_index("objectId")?;
    let g_index = column_index("g_cModelFlux")?;
    let r_index = column_index("r_cModelFlux")?;
    let i_index = column_index("i_cModelFlux")?;
    let z_index = column_index("z_cModelFlux")?;
    let y_index = column_index("y_cModelFlux")?;
    let u_index = column_index("u_cModelFlux")?;
    let shape_xx_index = column_index("shape_xx")?;
    let shape_yy_index = column_index("shape_yy")?;

    let mut galaxies = Vec::new();
    for record in reader.records() {
        let row = record?;
        let Some(object_id) = row.get(object_id_index) else {
            continue;
        };
        let (
            Some(g_flux),
            Some(r_flux),
            Some(i_flux),
            Some(z_flux),
            Some(y_flux),
            Some(u_flux),
            Some(shape_xx),
            Some(shape_yy),
        ) = (
            parse_float(&row, g_index),
            parse_float(&row, r_index),
            parse_float(&row, i_index),
            parse_float(&row, z_index),
            parse_float(&row, y_index),
            parse_float(&row, u_index),
            parse_float(&row, shape_xx_index),
            parse_float(&row, shape_yy_index),
        )
        else {
            continue;
        };

        galaxies.push(Galaxy {
            object_id: object_id.to_string(),
            g_flux,
            r_flux,
            i_flux,
            z_flux,
            y_flux,
            u_flux,
            shape_xx,
            shape_yy,
        });
    }

    Ok(galaxies)
}

fn parse_float(record: &csv::StringRecord, index: usize) -> Option<f64> {
    record.get(index)?.parse().ok()
}
