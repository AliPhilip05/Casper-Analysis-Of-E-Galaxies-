use csv::{ReaderBuilder, WriterBuilder};
use chrono::Local;
// use std::collections::HashMap;
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug)]
struct Galaxy {
    object_id: String,
    g_flux: f64,
    r_flux: f64,
    i_flux: f64,
    z_flux: f64,
    y_flux: f64,
    u_flux: f64,
    shape_xx: f64,
    shape_yy: f64,
}


fn file_reader() -> Result<(), Box<dyn std::error::Error>> {

    // ---------------------------------------------------------
    // READ CSV
    // ---------------------------------------------------------

    let csv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("input")
        .join("selected_galaxies_dp1.csv");

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_path(csv_path)?;

    let headers = reader.headers()?.clone();

    let get_index = |name: &str| -> Result<usize, Box<dyn std::error::Error>> {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("Missing column: {}", name).into())
    };

    let object_id_idx = get_index("objectId")?;
    let g_idx = get_index("g_cModelFlux")?;
    let r_idx = get_index("r_cModelFlux")?;
    let i_idx = get_index("i_cModelFlux")?;
    let z_idx = get_index("z_cModelFlux")?;
    let y_idx = get_index("y_cModelFlux")?;
    let u_idx = get_index("u_cModelFlux")?;
    let xx_idx = get_index("shape_xx")?;
    let yy_idx = get_index("shape_yy")?;

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for record in reader.records() {
        let row = record?;

        let galaxy = Galaxy {
            object_id: row[object_id_idx].to_string(),

            g_flux: match row[g_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            r_flux: match row[r_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            i_flux: match row[i_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            z_flux: match row[z_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            y_flux: match row[y_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            u_flux: match row[u_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            shape_xx: match row[xx_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },

            shape_yy: match row[yy_idx].parse() {
                Ok(v) => v,
                Err(_) => continue,
            },
        };

        galaxies.push(galaxy);
    }

    println!("Loaded {} galaxies", galaxies.len());

    // ---------------------------------------------------------
    // SHAPE ARRAYS
    // ---------------------------------------------------------

    let shape_xx_arr: Vec<f64> =
        galaxies.iter().map(|g| g.shape_xx).collect();

    let shape_yy_arr: Vec<f64> =
        galaxies.iter().map(|g| g.shape_yy).collect();

    // ---------------------------------------------------------
    // BANDS
    // ---------------------------------------------------------

    let bands: [(&str, fn(&Galaxy) -> f64); 6] = [
        ("u", |g| g.u_flux),
        ("g", |g| g.g_flux),
        ("r", |g| g.r_flux),
        ("i", |g| g.i_flux),
        ("z", |g| g.z_flux),
        ("y", |g| g.y_flux),
    ];    // ---------------------------------------------------------
    // OUTPUT DATA
    // ---------------------------------------------------------

    let mut all_data: Vec<(usize, String, String, f64, f64, f64)> =
        Vec::new();

    // ---------------------------------------------------------
    // CALCULATE CAS
    // ---------------------------------------------------------

    for (band, get_flux) in bands.iter() {

        // Get flux for this band
        let flux_array: Vec<f64> =
            galaxies.iter().map(|g| get_flux(g)).collect();

        // Equivalent to:
        // flux_array[::-1]
        let flux_180: Vec<f64> =
            flux_array.iter().rev().copied().collect();

        // Calculate radii
        let (r_80_arr, r_20_arr) =
            estimate_radii(
                &flux_array,
                &shape_xx_arr,
                &shape_yy_arr,
            );

        // Calculate CAS for every galaxy
        for idx in 0..galaxies.len() {

            let galaxy = &galaxies[idx];

            let flux_val = flux_array[idx];

            // -------------------------
            // C - Concentration
            // -------------------------

            let c = calculate_c(
                r_80_arr[idx],
                r_20_arr[idx],
            );

            // -------------------------
            // A - Asymmetry
            // -------------------------

            let denominator =
                flux_val.abs() + flux_180[idx].abs();

            let a = if denominator != 0.0 {
                (flux_val - flux_180[idx]).abs() / denominator
            } else {
                0.0
            };

            // -------------------------
            // S - Smoothness
            // -------------------------

            let mut filter_size =
                (0.3 * r_80_arr[idx]).round() as usize;

            filter_size = filter_size.max(3);

            // Make sure filter size is odd
            if filter_size % 2 == 0 {
                filter_size += 1;
            }

            let i_s =
                uniform_filter(&flux_array, filter_size);

            let s = if flux_val != 0.0 {
                (flux_val - i_s[idx]).abs() / flux_val.abs()
            } else {
                0.0
            };

            // -------------------------
            // Store result
            // -------------------------

            all_data.push((
                idx + 1,
                galaxy.object_id.clone(),
                band.to_string(),
                (c * 10000.0).round() / 10000.0,
                (a * 10000.0).round() / 10000.0,
                (s * 10000.0).round() / 10000.0,
            ));
        }
    }

    // ---------------------------------------------------------
    // CREATE OUTPUT FILENAME
    // ---------------------------------------------------------

    let timestamp =
        Local::now().format("%Y%m%d_%H%M%S");

    let file_name =
        format!("cas_results_{}.csv", timestamp);

    let output_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("results");
    fs::create_dir_all(&output_folder)?;

    let output_path = output_folder.join(&file_name);

    // ---------------------------------------------------------
    // WRITE CSV
    // ---------------------------------------------------------

    let file = File::create(&output_path)?;

    let mut writer =
        WriterBuilder::new().from_writer(file);

    writer.write_record([
        "index",
        "objectId",
        "band",
        "C",
        "A",
        "S",
    ])?;

    for (index, object_id, band, c, a, s) in all_data {
        writer.write_record([
            index.to_string(),
            object_id,
            band,
            c.to_string(),
            a.to_string(),
            s.to_string(),
        ])?;
    }

    writer.flush()?;

    // ---------------------------------------------------------
    // SAVE LATEST FILE
    // ---------------------------------------------------------

    fs::write(
        output_folder.join("latest_file.txt"),
        &file_name,
    )?;

    // ---------------------------------------------------------
    // DONE
    // ---------------------------------------------------------

    println!("Saved to {}", output_path.display());

    Ok(())
}
