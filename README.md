Multi-Light-Band CAS Analysis of E+A Galaxies
Concentration, Asymmetry, and Smoothness Analysis Using Data from the Vera C. Rubin Observatory's Legacy Survey of Space and Time (LSST)
Isabella Troy Brazoban, Lorik Fazliu, and Ali Philip
Advisor: Dr. Charles Liu
The City University of New York — College of Staten Island
Department of Physics and Astronomy
2800 Victory Blvd, Staten Island, NY 10314
Overview
This project develops an automated pipeline for analyzing the morphology of post-starburst (E+A) galaxies using data from the Vera C. Rubin Observatory's Legacy Survey of Space and Time (LSST).
Galaxy morphology provides important information about how galaxies form and evolve. In particular, disturbed morphologies can provide evidence of galaxy mergers and interactions. The Concentration, Asymmetry, and Smoothness (CAS) framework, introduced by Conselice (2003), provides a quantitative method for characterizing these morphological features.

The goal of this project is to adapt the CAS framework to the scale of LSST data and create a computationally efficient workflow capable of processing large galaxy samples across multiple photometric bands.

The pipeline currently analyzes six optical and near-infrared bands:

u
g
r
i
z
y
The resulting CAS measurements can be used to investigate the structural properties of E+A galaxies and, ultimately, help identify possible merger remnants.
Scientific Motivation
The Vera C. Rubin Observatory's LSST will produce approximately 20 terabytes of raw astronomical data per night, creating both an unprecedented scientific opportunity and a computational challenge.
Traditional approaches to galaxy morphology often rely on manually examining individual galaxies or processing relatively small datasets. Such approaches do not scale efficiently to LSST-sized surveys.

This project therefore focuses on:

Selecting candidate E+A galaxies from survey catalog data.
Reducing the relevant data to a manageable local dataset.
Calculating CAS parameters automatically.
Performing the analysis across multiple photometric bands.
Saving the resulting measurements in a format suitable for further analysis.
The broader scientific objective is to use these measurements to study the relationship between galaxy structure, stellar populations, mergers, and galactic evolution.
E+A Galaxy Selection
The initial catalog query retrieves up to 100,000 objects from the LSST Data Preview 0.2 (DP0.2) object catalog.
Candidate galaxies are then selected using photometric criteria designed to isolate the transition region between star-forming and quiescent galaxies, commonly referred to as the Green Valley.

The selection criteria include:

u-band magnitude
15.0
≤
u
≤
21.0

u − g color
1.0
≤
u
−
g
≤
1.8

g − r color
0.3
≤
g
−
r
≤
0.85

Applying these filters reduced the initial sample of approximately 100,000 objects to 103 target galaxies.

Data Source
The project uses catalog data accessed through the Rubin Science Platform (RSP) and its Table Access Protocol (TAP) service.
The analysis uses measurements including:

Object IDs
Right ascension and declination
cModel fluxes in the six LSST bands
Shape moments
Galaxy size measurements
The primary catalog queried by the current implementation is:
dp02_dc2_catalogs.Object

An example TAP query is:
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

The project also queries TAP_SCHEMA.columns to inspect the available catalog columns and their descriptions.
CAS Analysis
The pipeline calculates three morphological parameters:
Concentration (C)
Asymmetry (A)
Smoothness (S)
These parameters are calculated independently for each galaxy and each photometric band.
The final dataset therefore contains CAS measurements across the:

u, g, r, i, z, y

bands.
Concentration
Concentration describes how centrally concentrated the galaxy's light is.
The implementation uses the ratio between the radii containing 80% and 20% of the estimated flux:

C
=
5
log
⁡
10
(
r
80
r
20
)

where:

r
80
 is the estimated radius containing 80% of the light.
r
20
 is the estimated radius containing 20% of the light.
Higher concentration values indicate that the galaxy's light is more strongly concentrated toward its center.
The code protects against division by zero by assigning a value of 0.0 when 
r
20
=
0
.

Asymmetry
Asymmetry measures how different a galaxy is from itself after a 180-degree rotation.
Conceptually, the original flux distribution is compared with its rotated counterpart:

A
=
∑
∣
I
−
I
180
∣
∑
∣
I
∣
−
A
b
k
g

where:

I
 is the original flux distribution.
I
180
 is the flux distribution rotated by 180 degrees.
A
b
k
g
 is the background correction.
A larger asymmetry value generally indicates a more disturbed or irregular morphology, which can be associated with interactions or mergers.
The current implementation uses an array-reversal approach as an approximation to the 180-degree comparison.

Smoothness
Smoothness measures small-scale structural variation within the galaxy.
The original flux distribution is compared with a smoothed version produced using a uniform filter:

S
=
∑
∣
I
−
I
S
∣
∑
∣
I
∣
−
S
b
k
g

where 
I
S
 represents the smoothed flux distribution.

The smoothing scale is dynamically determined from the estimated Petrosian/galaxy radius:

filterSize = max(3, int(round(0.3 * rPetea)))

The filter size is also forced to be odd:
if filterSize % 2 == 0:
    filterSize += 1

This prevents invalid filter sizes and ensures that even very small objects can be processed.
Higher smoothness values indicate greater small-scale structure or clumpiness.

Galaxy Size Estimation
The pipeline estimates a characteristic galaxy radius using the second-order shape moments:
r_total = np.sqrt(np.abs(shape_xx + shape_yy))

The trace of the shape-moment tensor is used as an approximation of the spatial extent of the galaxy.
The normalized flux is then used to estimate 
r
80
 and 
r
20
:

flux_norm = (flux - min_flux) / flux_range

The current implementation uses:
r_80 = r_total * (0.5 + 0.5 * flux_norm)
r_20 = r_total * (0.1 + 0.3 * flux_norm)

Small positive lower bounds are applied to the radii to prevent division-by-zero errors.
Note: These radius relationships are part of the current preliminary implementation and are intended as an approximation for the automated pipeline. A future version should validate the estimated radii against direct curve-of-growth or Petrosian-radius measurements.
Pipeline Workflow
The overall workflow is:
LSST / DP0.2 Catalog
        │
        ▼
TAP Query
        │
        ▼
Initial Galaxy Sample
        │
        ▼
Photometric Selection
(u, u-g, g-r)
        │
        ▼
E+A / Green Valley Candidates
        │
        ▼
Extract Flux + Shape Data
        │
        ▼
Process u, g, r, i, z, y Bands
        │
        ├──► Estimate r80 and r20
        │
        ├──► Calculate Concentration
        │
        ├──► Calculate Asymmetry
        │
        └──► Calculate Smoothness
        │
        ▼
CAS Results
        │
        ▼
CSV Output

Project Structure
A recommended project structure is:
CAS-EA-Galaxy-Analysis/
│
├── data.input/
│   └── FINAL LIST.csv
│
├── scripts/
│   └── cas_analysis.py
│
├── output/
│   └── cas_results_YYYYMMDD_HHMMSS.csv
│
├── dc2_objects.txt
│
├── latest_file.txt
│
└── README.md

The exact directory structure may differ depending on the user's Rubin Science Platform environment.
Requirements
The pipeline requires Python and the following packages:
numpy
pandas
scipy
matplotlib
lsst.rsp
lsst.daf.butler
lsst.geom
lsst.afw.display

The LSST-specific packages are expected to be available within an appropriate Rubin Science Platform / LSST Science Pipelines environment.
Installation
Clone the repository:
git clone <repository-url>
cd CAS-EA-Galaxy-Analysis

Install the standard Python dependencies if they are not already available:
pip install numpy pandas scipy matplotlib

LSST/Rubin-specific packages should be installed and configured through the appropriate Rubin Science Platform environment rather than a standard pip installation.
Input Data
The CAS processing script expects the selected galaxy sample to be stored as:
data.input/FINAL LIST.csv

The CSV file should contain the following fields:
objectId
g_cModelFlux
r_cModelFlux
i_cModelFlux
z_cModelFlux
y_cModelFlux
u_cModelFlux
shape_xx
shape_yy

The script reads these values and converts the numerical fields to floating-point values before performing the CAS calculations.
Rows containing invalid numerical values are skipped.

Running the Analysis
Run the CAS analysis script from the project environment:
python cas_analysis.py

The script processes each galaxy across all six bands:
u
g
r
i
z
y

For each galaxy/band combination, the following quantities are calculated:
C
A
S

Output
The results are saved automatically as a timestamped CSV file:
cas_results_YYYYMMDD_HHMMSS.csv

For example:
cas_results_20260913_160700.csv

The output contains:
Column	Description
index	Index of the galaxy in the input sample
objectId	LSST catalog object identifier
band	Photometric band
C	Concentration
A	Asymmetry
S	Smoothness

Example:
index,objectId,band,C,A,S
1,123456789,g,3.2145,0.0872,0.1421
2,987654321,g,2.9183,0.1134,0.2017
3,456789123,r,3.4521,0.0721,0.1288

The name of the most recently generated file is also stored in:
latest_file.txt

Important Implementation Notes
Background Corrections
The theoretical CAS definitions include background corrections:
A_bkg
S_bkg

The current production loop calculates the asymmetry and smoothness values directly from the galaxy flux arrays without explicitly subtracting separately estimated background values.
Future versions should incorporate robust background estimates from blank-sky regions or dedicated background measurements.

180-Degree Rotation
The current implementation approximates the 180-degree comparison using:
flux_180 = flux_array[::-1]

This reverses the order of the one-dimensional catalog flux array.
For a true image-based CAS analysis, the 180-degree transformation should instead be performed on the two-dimensional galaxy image around the galaxy's center.

Smoothness
The current implementation applies the uniform filter to the full flux array:
I_S = ndimage.uniform_filter(flux_array, size=filterSize)

A future image-based implementation should smooth the 2D pixel-level galaxy image, rather than a one-dimensional array of catalog fluxes.
These distinctions are important when comparing the current preliminary pipeline to the original Conselice CAS methodology.

Current Limitations
This project is an ongoing preliminary research effort. Several components of the pipeline are currently approximations designed to establish an automated and scalable framework.
Important limitations include:

The current CAS calculations operate primarily on catalog-level flux measurements rather than full 2D galaxy images.
The 
r
80
 and 
r
20
 relationships are approximate.
The 180-degree asymmetry calculation should ultimately use 2D image rotation.
Background corrections require further development.
The sample-selection criteria identify a Green Valley population and are not, by themselves, a definitive spectroscopic classification of E+A galaxies.
The current sample is relatively small compared with the eventual scale of LSST.
CAS measurements require further validation against established implementations and simulated or observationally confirmed merger samples.
Future Work
The next stages of the project will focus on improving both the scientific accuracy and computational scalability of the pipeline.
Potential improvements include:

Implementing true 2D image-based CAS measurements.
Calculating Petrosian radii directly from galaxy images.
Improving background estimation and subtraction.
Validating CAS measurements against the original Conselice methodology.
Comparing CAS measurements between the six LSST bands.
Expanding the galaxy sample beyond the initial 103 candidates.
Automating the identification of merger remnants.
Investigating relationships between morphology, stellar mass, age, and galaxy evolution.
Optimizing the pipeline for significantly larger LSST datasets.
Developing visualization tools for CAS parameter distributions.
Comparing automated classifications with visually classified galaxies.
Scientific Goal
The ultimate goal is to establish a scalable framework for studying the morphology of post-starburst galaxies using LSST data.
By measuring Concentration, Asymmetry, and Smoothness across multiple wavelengths, the project aims to investigate whether structural signatures can reveal the remnants of galaxy mergers and interactions.

Understanding these morphological signatures may help answer broader questions about:

How galaxy mergers trigger or terminate star formation.
How post-starburst galaxies evolve.
How stellar mass and age relate to galaxy morphology.
How merger remnants appear across different wavelengths.
How frequently mergers contribute to galaxy evolution.
The development of an automated CAS pipeline provides a foundation for extending these analyses to the much larger datasets expected from the Rubin Observatory's LSST.
References
Conselice, C. J. (2003). The Relationship between Stellar Light Distributions of Galaxies and Their Formation Histories. The Astrophysical Journal Supplement Series, 147, 1–28.
Vera C. Rubin Observatory. Legacy Survey of Space and Time (LSST).

Rubin Observatory. Rubin Science Platform / Data Preview 0.2 (DP0.2).

Authors
Isabella Troy Brazoban
Lorik Fazliu
Ali Philip
Advisor: Dr. Charles Liu

Department of Physics and Astronomy
The City University of New York — College of Staten Island

Status
Project Status: Ongoing / Preliminary Research
This repository contains an evolving research pipeline. Methods and implementations may change as the CAS analysis is validated and expanded to larger LSST datasets.
