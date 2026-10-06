# CAS Analysis — Rust Function Documentation

## Overview

This project calculates the **CAS (Concentration, Asymmetry, Smoothness)** parameters for galaxies from a CSV dataset.

The program:

1. Reads galaxy data from `data.input/FINAL LIST.csv`.
2. Extracts flux and shape-moment measurements.
3. Calculates estimated galaxy radii.
4. Calculates the **Concentration (C)** parameter.
5. Calculates the **Asymmetry (A)** parameter.
6. Calculates the **Smoothness (S)** parameter.
7. Writes the results to a timestamped CSV file.
8. Records the most recently generated filename in `latest_file.txt`.

The main calculations are divided into several functions so that each part of the analysis can be reused independently.

---

# Functions

## `calculate_c`

```ru[118;1:3ust
fn calculate_c(r_80: f64, r_20: f64) -> f64
```

### Description

Calculates the **Concentration (C)** parameter of a galaxy.

The concentration is calculated using the ratio between the radius containing approximately 80% of the galaxy's light (`r_80`) and the radius containing approximately 20% of the galaxy's light (`r_20`).

The formula used is:

\[
C = 5\log_{10}\left(\frac{r_{80}}{r_{20}}\right)
\]

A higher concentration value indicates that the galaxy's light is more concentrated toward its center.

### Parameters

| Parameter | Type  | Description                                            |
| --------- | ----- | ------------------------------------------------------ |
| `r_80`    | `f64` | Estimated radius containing 80% of the galaxy's light. |
| `r_20`    | `f64` | Estimated radius containing 20% of the galaxy's light. |

### Returns

`f64`

Returns the calculated concentration value.

If `r_20` is `0.0`, the function returns `0.0` to prevent division by zero.

### Example

```rust
let c = calculate_c(8.0, 2.0);

println!("Concentration: {}", c);
```

---

# `estimate_radii`

```rust
fn estimate_radii(
    flux_array: &[f64],
    shape_xx_arr: &[f64],
    shape_yy_arr: &[f64],
) -> (Vec<f64>, Vec<f64>)
```

### Description

Estimates the `r_80` and `r_20` radii for every galaxy in the dataset.

The function first calculates the approximate total size of each galaxy using its shape moments:

\[
r_{total} = \sqrt{|shape_{xx} + shape_{yy}|}
\]

The flux values are then normalized between `0` and `1`.

The normalized flux is used to estimate `r_80` and `r_20`:

\[
r_{80} = r_{total}(0.5 + 0.5F_{norm})
\]

\[
r_{20} = r_{total}(0.1 + 0.3F_{norm})
\]

Very small radii are clamped to `1e-6` to prevent numerical problems later in the calculations.

### Parameters

| Parameter      | Type     | Description                                                 |
| -------------- | -------- | ----------------------------------------------------------- |
| `flux_array`   | `&[f64]` | Flux measurements for all galaxies in one photometric band. |
| `shape_xx_arr` | `&[f64]` | `shape_xx` measurements for all galaxies.                   |
| `shape_yy_arr` | `&[f64]` | `shape_yy` measurements for all galaxies.                   |

### Returns

```rust
(Vec<f64>, Vec<f64>)
```

The returned tuple contains:

```text
(r_80_array, r_20_array)
```

where each vector contains one radius for every galaxy.

### Example

```rust
let (r_80, r_20) = estimate_radii(
    &flux_array,
    &shape_xx,
    &shape_yy,
);
```

---

# `uniform_filter`

```rust
fn uniform_filter(
    array: &[f64],
    size: usize,
) -> Vec<f64>
```

### Description

Applies a **moving-average filter** to an array.

This function is the Rust equivalent of the SciPy operation:

```python
ndimage.uniform_filter(array, size=size)
```

For every element, the function calculates the average of the surrounding values within the specified filter window.

For example, given:

```text
[2, 4, 6, 8, 10]
```

with a filter size of `3`, the function calculates local averages using neighboring values.

The filter is used when calculating the **Smoothness (S)** parameter.

### Parameters

| Parameter | Type     | Description                         |
| --------- | -------- | ----------------------------------- |
| `array`   | `&[f64]` | Input array containing flux values. |
| `size`    | `usize`  | Size of the moving-average window.  |

### Returns

```rust
Vec<f64>
```

Returns a new vector containing the smoothed values.

The original array is not modified.

### Edge Handling

When the filter reaches the beginning or end of the array, the function automatically reduces the window size so that it stays within the array boundaries.

For example:

```text
Array:
[1, 2, 3, 4, 5]

At index 0:
[1, 2]

At index 2:
[1, 2, 3, 4, 5]
```

depending on the requested filter size.

### Example

```rust
let flux = vec![
    10.0,
    20.0,
    30.0,
    40.0,
    50.0,
];

let smoothed = uniform_filter(&flux, 3);
```

---

# `main`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```

### Description

The `main` function controls the entire CAS analysis pipeline.

It is responsible for reading the input dataset, processing each galaxy, calculating the CAS parameters, and saving the results.

---

## Step 1 — Read the CSV

The program opens:

```text
data.input/FINAL LIST.csv
```

using Rust's `csv` crate.

```rust
let mut reader = ReaderBuilder::new()
    .flexible(true)
    .from_path(csv_path)?;
```

The CSV headers are then read so that the program can locate the required columns.

---

## Step 2 — Locate Required Columns

The program searches for the following columns:

```text
objectId
g_cModelFlux
r_cModelFlux
i_cModelFlux
z_cModelFlux
y_cModelFlux
u_cModelFlux
shape_xx
shape_yy
```

The column indexes are stored so they can be accessed efficiently while processing each row.

---

## Step 3 — Create Galaxy Structures

Every valid CSV row is converted into a `Galaxy` structure:

```rust
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
```

Rows containing invalid numerical values are skipped.

---

## Step 4 — Create Shape Arrays

The program extracts the shape moments from every galaxy:

```rust
let shape_xx_arr: Vec<f64> =
    galaxies.iter().map(|g| g.shape_xx).collect();

let shape_yy_arr: Vec<f64> =
    galaxies.iter().map(|g| g.shape_yy).collect();
```

These arrays are reused for every photometric band.

---

## Step 5 — Process Each Photometric Band

The program processes six photometric bands:

```text
u
g
r
i
z
y
```

Each band corresponds to a different flux measurement.

For example:

```rust
("g", |g: &Galaxy| g.g_flux)
```

means that the `g` band uses the `g_flux` value from each galaxy.

---

## Step 6 — Create the Flux Array

For every band, the program creates an array containing the flux of every galaxy.

```rust
let flux_array: Vec<f64> =
    galaxies.iter().map(|g| get_flux(g)).collect();
```

For example:

```text
Galaxy 1 → 100
Galaxy 2 → 150
Galaxy 3 → 200
...
```

becomes:

```text
[100, 150, 200, ...]
```

---

## Step 7 — Create the Reversed Flux Array

The program creates a reversed version of the flux array:

```rust
let flux_180: Vec<f64> =
    flux_array.iter().rev().copied().collect();
```

This corresponds to the Python operation:

```python
flux_array[::-1]
```

The reversed array is used when calculating the asymmetry value.

> **Note:** This is a direct translation of the original Python code. Reversing a 1D list is not equivalent to physically rotating a 2D galaxy image by 180°. A scientifically rigorous CAS implementation would normally perform the rotation on image pixels.

---

## Step 8 — Estimate Galaxy Radii

The program calculates `r_80` and `r_20`:

```rust
let (r_80_arr, r_20_arr) =
    estimate_radii(
        &flux_array,
        &shape_xx_arr,
        &shape_yy_arr,
    );
```

These values are then used by the concentration calculation.

---

# CAS Calculations

## Concentration

For every galaxy:

```rust
let c = calculate_c(
    r_80_arr[idx],
    r_20_arr[idx],
);
```

This calculates:

\[
C = 5\log_{10}(r_{80}/r_{20})
\]

---

## Asymmetry

The asymmetry is calculated using the original flux and the reversed flux:

```rust
let denominator =
    flux_val.abs() + flux_180[idx].abs();

let a = if denominator != 0.0 {
    (flux_val - flux_180[idx]).abs() / denominator
} else {
    0.0
};
```

The calculation is:

\[
A =
\frac{|I-I_{180}|}
{|I|+|I_{180}|}
\]

If the denominator is zero, the result is set to `0.0`.

---

## Smoothness

The filter size is determined from `r_80`:

```rust
let mut filter_size =
    (0.3 * r_80_arr[idx]).round() as usize;
```

The minimum filter size is `3`:

```rust
filter_size = filter_size.max(3);
```

If the filter size is even, it is increased by one:

```rust
if filter_size % 2 == 0 {
    filter_size += 1;
}
```

The flux array is then smoothed:

```rust
let i_s =
    uniform_filter(&flux_array, filter_size);
```

The smoothness calculation is:

```rust
let s = if flux_val != 0.0 {
    (flux_val - i_s[idx]).abs() / flux_val.abs()
} else {
    0.0
};
```

Conceptually:

\[
S =
\frac{|I-I_S|}
{|I|}
\]

where:

- `I` = original flux
- `I_S` = smoothed flux

---

# Output

After calculating `C`, `A`, and `S`, the program stores the results:

```rust
all_data.push((
    idx + 1,
    galaxy.object_id.clone(),
    band.to_string(),
    c,
    a,
    s,
));
```

Each result contains:

| Field      | Description                                         |
| ---------- | --------------------------------------------------- |
| `index`    | Position of the galaxy in the input dataset.        |
| `objectId` | Galaxy's object identifier.                         |
| `band`     | Photometric band (`u`, `g`, `r`, `i`, `z`, or `y`). |
| `C`        | Concentration value.                                |
| `A`        | Asymmetry value.                                    |
| `S`        | Smoothness value.                                   |

---

# Timestamped Output File

The program generates a unique filename using the current date and time:

```rust
let timestamp =
    Local::now().format("%Y%m%d_%H%M%S");
```

For example:

```text
cas_results_20261005_194530.csv
```

This prevents previous analysis results from being overwritten.

---

# CSV Output

The results are written using the `csv` crate:

```rust
let mut writer =
    WriterBuilder::new().from_writer(file);
```

The output CSV contains:

```text
index,objectId,band,C,A,S
```

Example:

```text
1,123456789,u,2.3142,0.1023,0.0541
2,123456790,u,2.8123,0.0821,0.0412
3,123456791,u,3.1024,0.1523,0.0732
```

---

# `latest_file.txt`

After creating the output file, the program writes its filename to:

```text
latest_file.txt
```

For example:

```text
cas_results_20261005_194530.csv
```

This allows other parts of the project to determine which CAS results file was generated most recently.

---

# Error Handling

The `main` function returns:

```rust
Result<(), Box<dyn std::error::Error>>
```

This allows errors from file operations, CSV parsing, and other operations to be propagated using the `?` operator.

For example:

```rust
let file = File::create(&output_path)?;
```

If the file cannot be created, the error is automatically returned from `main`.

This is preferable to manually checking every possible error.

---

# Dependencies

The project uses the following external crates:

## `csv`

Used for reading and writing CSV files.

```toml
csv = "1.3"
```

## `chrono`

Used to generate timestamps for output filenames.

```toml
chrono = "0.4"
```

---

# Function Summary

| Function           | Purpose                                      |
| ------------------ | -------------------------------------------- |
| `calculate_c()`    | Calculates galaxy concentration.             |
| `estimate_radii()` | Estimates `r_80` and `r_20` for each galaxy. |
| `uniform_filter()` | Performs moving-average smoothing.           |
| `main()`           | Runs the complete CAS analysis pipeline.     |

---

# Program Flow

```text
                    ┌──────────────────────┐
                    │ FINAL LIST.csv       │
                    └──────────┬───────────┘
                               │
                               ▼
                    ┌──────────────────────┐
                    │ Read CSV             │
                    │ Create Galaxy structs│
                    └──────────┬───────────┘
                               │
                               ▼
                    ┌──────────────────────┐
                    │ Extract flux +       │
                    │ shape moment arrays  │
                    └──────────┬───────────┘
                               │
                               ▼
                     ┌────────────────────┐
                     │ Process u,g,r,i,z,y│
                     └─────────┬──────────┘
                               │
                 ┌─────────────┼─────────────┐
                 ▼             ▼             ▼
          ┌────────────┐ ┌────────────┐ ┌────────────┐
          │ Estimate   │ │ Calculate  │ │ Smooth     │
          │ Radii      │ │ C / A      │ │ Flux       │
          │ r80 / r20  │ │            │ │ Calculate S│
          └──────┬─────┘ └─────┬──────┘ └─────┬──────┘
                 │              │              │
                 └──────────────┼──────────────┘
                                ▼
                    ┌──────────────────────┐
                    │ Store CAS Results    │
                    └──────────┬───────────┘
                               │
                               ▼
                    ┌──────────────────────┐
                    │ Write results CSV    │
                    └──────────┬───────────┘
                               │
                    ┌──────────┴───────────┐
                    ▼                      ▼
             cas_results_*.csv       latest_file.txt
```

---

# Running the Program

From the project directory:

```bash
cargo run
```

The program should produce output similar to:

```text
Loaded 100 galaxies
Saved to ./cas_results_20261005_194530.csv
```

The generated files will be:

```text
cas_results_20261005_194530.csv
latest_file.txt
```

---

# Important Scientific Note

The current Rust implementation intentionally follows the behavior of the provided Python program.

In particular, the asymmetry calculation currently uses:

```rust
flux_array.iter().rev()
```

as the `180°` comparison.

For actual astronomical CAS analysis, the asymmetry calculation should instead operate on **2D galaxy images**, rotate each image by 180°, compare the original and rotated images, and apply the appropriate background correction.

Similarly, the current `estimate_radii()` function provides an **estimated radius based on flux and shape moments**, rather than directly measuring the radii enclosing 20% and 80% of the integrated galaxy light.

Therefore, this implementation should be considered a **Rust translation of the existing analysis algorithm**, rather than a fully physically rigorous implementation of the astronomical CAS methodology.
