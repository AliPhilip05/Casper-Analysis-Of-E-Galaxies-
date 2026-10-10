# Multi-band CAS Analysis of E+A Galaxy Candidates

**Concentration, Asymmetry, and Smoothness (CAS) analysis using data from the Vera C. Rubin Observatory’s Legacy Survey of Space and Time (LSST)**

**Research authors:** Isabella Troy Brazoban, Lorik Fazliu, and Ali Philip

**Advisor:** Dr. Charles Liu

Department of Physics and Astronomy, The City University of New York — College of Staten Island

2800 Victory Blvd, Staten Island, NY 10314

> **Project status:** Ongoing preliminary research. The current measurements and methods are prototypes and require further validation.

## Overview

This project develops an automated workflow for studying the morphology of candidate post-starburst (E+A) galaxies. It applies the Concentration, Asymmetry, and Smoothness (CAS) framework across six LSST photometric bands: **u, g, r, i, z, and y**.

Galaxy structure can preserve evidence of formation and evolution. In particular, disturbed morphologies may indicate mergers or interactions. The project’s longer-term goal is to use multi-band CAS measurements to investigate the structural properties of E+A candidates and identify possible merger remnants at survey scale.

The workflow is designed to:

1. Retrieve object measurements from Rubin survey catalogs.
2. Select a manageable Green Valley candidate sample using photometric cuts.
3. Calculate CAS values across the six bands.
4. Save the measurements in a CSV format for further analysis.

## Candidate selection

The initial catalog query can retrieve up to approximately 100,000 objects from the LSST Data Preview 0.2 (DP0.2) object catalog. Photometric cuts are then used to select Green Valley candidates:

| Measurement | Selection range |
| --- | ---: |
| u-band magnitude | 15.0 ≤ u ≤ 21.0 |
| u − g color | 1.0 ≤ u − g ≤ 1.8 |
| g − r color | 0.3 ≤ g − r ≤ 0.85 |

Applying these cuts produced an initial sample of 103 candidates. These photometric criteria identify Green Valley objects; they do **not**, by themselves, confirm a spectroscopic E+A classification.

## Data source

Catalog measurements are accessed through the Rubin Science Platform (RSP) Table Access Protocol (TAP) service. The primary catalog used by the current query workflow is `dp02_dc2_catalogs.Object`.

Measurements used by the analysis include object IDs, coordinates, cModel fluxes in the six bands, shape moments, and galaxy-size estimates. The project also queries `TAP_SCHEMA.columns` to inspect available columns and descriptions.

Example TAP query:

```sql
SELECT TOP 10000
    objectId,
    coord_ra,
    coord_dec,
    g_cModelFlux,
    r_cModelFlux,
    i_cModelFlux,
    g_kronRad,
    r_kronRad,
    i_kronRad,
    shape_xx,
    shape_xy,
    shape_yy,
    refExtendedness
FROM dp02_dc2_catalogs.Object
WHERE detect_isPrimary = 1
```

The query is an example of catalog retrieval; the photometric selection is applied as a later workflow step.

## CAS measurements

The pipeline calculates three quantities independently for each galaxy and photometric band.

| Parameter | Meaning | General interpretation |
| --- | --- | --- |
| **C — Concentration** | Central concentration of the galaxy’s light | Higher values indicate a more centrally concentrated light profile. |
| **A — Asymmetry** | Difference between a flux distribution and its 180°-rotated counterpart | Higher values can indicate a disturbed or irregular morphology. |
| **S — Smoothness** | Difference between the flux distribution and a smoothed version | Higher values indicate more small-scale structure or clumpiness. |

### Concentration

Concentration is estimated from the radii containing 80% and 20% of the flux:

$$C = 5\log_{10}\left(\frac{r_{80}}{r_{20}}\right)$$

Here, `r80` and `r20` are approximate radii in the current implementation. A zero `r20` is guarded against to avoid division by zero.

### Asymmetry

The conceptual CAS definition compares the original flux distribution, `I`, with its 180°-rotated version, `I₁₈₀`, and subtracts a background correction:

$$A = \frac{\sum |I-I_{180}|}{\sum |I|} - A_{\mathrm{bkg}}$$

The current prototype approximates the rotation by reversing a one-dimensional flux array (`flux_array[::-1]`). It does not explicitly estimate or subtract a separate background term.

### Smoothness

Smoothness compares the original flux distribution with a smoothed version, `Iₛ`:

$$S = \frac{\sum |I-I_S|}{\sum |I|} - S_{\mathrm{bkg}}$$

The filter scale is based on the estimated `r80` value, with a minimum size of 3 and an odd size:

```text
filter_size = max(3, round(0.3 * r80))
if filter_size % 2 == 0:
    filter_size += 1
```

The filter size is at least 3 and is made odd. In the current prototype, the uniform filter is applied to the one-dimensional flux array rather than to a two-dimensional image.

### Approximate radius estimation

The prototype estimates a characteristic radius from the trace of the shape-moment tensor. The following is pseudocode describing the approximation, not a runnable Python script:

```text
r_total = sqrt(abs(shape_xx + shape_yy))
flux_norm = (flux - min_flux) / flux_range
r_80 = r_total * (0.5 + 0.5 * flux_norm)
r_20 = r_total * (0.1 + 0.3 * flux_norm)
```

Small positive lower bounds are applied to avoid division-by-zero errors. These relationships are preliminary approximations; future work should validate radii against direct curve-of-growth or Petrosian-radius measurements.

## Workflow

```text
Rubin DP0.2 catalog
        ↓
TAP query and initial sample
        ↓
Photometric selection (u, u − g, g − r)
        ↓
Green Valley candidates
        ↓
Extract flux and shape measurements
        ↓
Process u, g, r, i, z, and y bands
        ├── Estimate r80 and r20
        ├── Calculate concentration (C)
        ├── Calculate asymmetry (A)
        └── Calculate smoothness (S)
        ↓
CAS results CSV
```

## Repository and implementation status

The repository is organized into three top-level areas: `frontend/` contains the static dashboard, `backend/` contains the Rust and Python analysis/query code plus its data, and `documents/` contains reports and technical notes. This repository also has a Rust 2021 Cargo project and an earlier Rubin Science Platform query workflow.

The current working tree is still being integrated: `backend/src/main.rs` does not yet invoke the CSV reader and CAS processing routine. The Rust workflow is therefore not yet a runnable, one-command pipeline. The earlier Python/RSP workflow is a separate prototype and requires the appropriate Rubin Science Platform environment.

Notes in `documents/technical/` include material from earlier iterations. Use this README for the current folder layout and input/output paths.

```text
frontend/                 Static dashboard
backend/                  Rust crate and query scripts
  src/                    Rust source
  query/                  Python/RSP query and analysis prototypes
  data/input/             Selected-galaxy input CSV
  data/results/           Generated CAS results and latest-file pointer
documents/                Research papers, reports, and technical notes
README.md                 Project overview and usage notes
```

### Rust dependencies

The Cargo project is in `backend/` and currently declares:

- Rust 2021 edition
- `csv` for CSV input and output
- `chrono` for timestamped output filenames

### Input CSV

The current Rust reader expects these columns:

```text
objectId
u_cModelFlux
g_cModelFlux
r_cModelFlux
i_cModelFlux
z_cModelFlux
y_cModelFlux
shape_xx
shape_yy
```

Rows with values that cannot be parsed as numbers are skipped. The input file is `backend/data/input/selected_galaxies_dp1.csv`. The Rust reader resolves it relative to the backend project directory.

### Results CSV

The analysis writes one row for each galaxy and band under `backend/data/results/`. The output filename follows this pattern:

```text
cas_results_YYYYMMDD_HHMMSS.csv
```

The columns are:

| Column | Description |
| --- | --- |
| `index` | Position of the galaxy in the input sample |
| `objectId` | LSST catalog object identifier |
| `band` | Photometric band (`u`, `g`, `r`, `i`, `z`, or `y`) |
| `C` | Concentration |
| `A` | Asymmetry |
| `S` | Smoothness |

Schema example (illustrative values):

```csv
index,objectId,band,C,A,S
1,123456789,u,3.2145,0.0872,0.1421
2,987654321,g,2.9183,0.1134,0.2017
3,456789123,r,3.4521,0.0721,0.1288
```

The latest output filename is also written to `backend/data/results/latest_file.txt`.

## Frontend

The frontend is a static HTML/CSS/JavaScript page in `frontend/` with no build step. It presents the project workflow, CAS definitions, an interactive measurement view, and a results table. It starts with clearly labeled illustrative preview values; load a results CSV to inspect real output. The browser reads the selected CSV locally and does not upload it to a server.

From the repository root, serve the static files with:

```bash
python3 -m http.server 8000
```

Then open <http://localhost:8000/frontend/>. The dashboard accepts a results CSV with `objectId`, `band`, `C`, `A`, and `S` columns; the `index` column is optional. It supports band filtering, object-ID search, sorting, pagination, and CSV export.

## Limitations

The current work is a preliminary research prototype. In particular:

- CAS calculations use catalog-level flux measurements rather than full two-dimensional galaxy images.
- The `r80` and `r20` relationships are approximate.
- Asymmetry uses a one-dimensional array reversal rather than rotating an image around the galaxy center.
- Smoothness is calculated on a one-dimensional flux array rather than a two-dimensional image.
- The current calculation loop does not explicitly subtract separately estimated background values for asymmetry or smoothness.
- Green Valley color cuts are not a definitive spectroscopic E+A classification.
- The initial sample of 103 candidates is small relative to the eventual scale of LSST.
- Measurements need validation against established CAS implementations and confirmed or simulated merger samples.

These differences matter when comparing this prototype with the original image-based CAS methodology.

## Future work

Planned improvements include:

- Implement true two-dimensional image-based CAS measurements and image rotation.
- Estimate Petrosian radii directly from galaxy images.
- Improve background estimation and subtraction.
- Validate measurements against the original Conselice methodology.
- Compare CAS measurements across all six LSST bands.
- Expand the candidate sample and optimize processing for larger datasets.
- Compare automated classifications with visually classified galaxies.
- Study links between morphology, stellar mass, stellar age, mergers, and galaxy evolution.

## Scientific goal

By measuring concentration, asymmetry, and smoothness across multiple wavelengths, this project aims to investigate whether structural signatures can reveal merger remnants among post-starburst galaxy candidates. The broader questions include how mergers affect star formation, how post-starburst galaxies evolve, and how merger remnants appear across different wavelengths.

## References

- Conselice, C. J. (2003). “The Relationship between Stellar Light Distributions of Galaxies and Their Formation Histories.” *The Astrophysical Journal Supplement Series*, 147, 1–28.
- Vera C. Rubin Observatory. *Legacy Survey of Space and Time (LSST).*
- Rubin Science Platform. *Data Preview 0.2 (DP0.2).*

## Contributions

- **Research concept, scientific content, and original project description:** Isabella Troy Brazoban, Lorik Fazliu, and Ali Philip.
- **Research advisor:** Dr. Charles Liu.
- **Frontend prompt clarification and frontend implementation:** OpenAI Codex, based on the human-authored project README and scientific description. Codex is credited for frontend assistance, not for the research, scientific results, or interpretation.

### Clarified frontend prompt

> Using the human-authored README and project description as the source of truth, create a responsive, accessible frontend for this repository’s multi-band CAS analysis of Green Valley/E+A galaxy candidates. Explain the scientific goal, candidate-selection criteria, workflow, CAS measurements, and current limitations. Provide an interactive view of results across the `u`, `g`, `r`, `i`, `z`, and `y` bands, with a table that can load and export CAS results CSV files. Clearly label any illustrative values and distinguish them from uploaded analysis results. Preserve the project’s preliminary status: explain the approximate radii, one-dimensional asymmetry and smoothness operations, missing explicit background subtraction, and the fact that photometric selection is not spectroscopic E+A confirmation. Do not invent scientific findings or present sample values as observations. Credit the human authors for the research and scientific content, and credit OpenAI Codex for clarifying this frontend prompt and assisting with frontend implementation.

The prompt was clarified by OpenAI Codex from the human-written README; the frontend implementation is credited separately from the scientific work.
