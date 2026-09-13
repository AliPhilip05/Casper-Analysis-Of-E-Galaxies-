import csv
import numpy as np
import scipy.ndimage as ndimage
import pandas as pd
from datetime import datetime
from pathlib import Path
import os

with open("data.input/FINAL LIST.csv", 'r') as file:
    content = file.read() #Opens the file and reads it (FR FR brodello) 


def calculateC(r_80, r_20): #Concentration Function (This says with shum pulp) <- Rare/niche sopranos quote pull
    if r_20 == 0: #protects from division by zero 
        return 0.0
    return 5.0 * np.log10(r_80 / r_20) #higher C means more concentrated towards the center 

def calculateA(I, I_180, A_bkg): #Asym Function where I is the Flux array and the I_180 is the rotated version and A_bkg is the background correction
    numerator = np.sum(np.abs(I - I_180)) #Flux rotated 180 *Cue MW2 edit* -> subtracts the rotated flux by original, takes abs and sums for total aysm flux 
    denominator = np.sum(np.abs(I)) # Like a normalizer so the result is dimensionless
    return (numerator / denominator) - A_bkg #subtracts from background noise-> higher A means more asymmetric galaxy 

def calculateS(I, rPetea, S_bkg): #Smoothness Function
    filterSize = max(3, int(round(0.3 * rPetea))) #LOISSSS im setting blur kernal set to 30% — min 3 so it always blurs
    if filterSize % 2 == 0:
        filterSize += 1 # if the result is even add 1 
    I_S = ndimage.uniform_filter(I, size=filterSize) #Blurred version from OG to find fine structure 
    numerator = np.sum(np.abs(I - I_S)) # subtracts blurred array from OG and takes abs val of sums
    denominator = np.sum(np.abs(I))
    return (numerator / denominator) - S_bkg #background noise

def estimate_radii(flux_array, shape_xx_arr, shape_yy_arr): # defines the function r_80,20 per galaxy per band 
    r_80_list = []
    r_20_list = [] # empty list for radii collection for each galaxy 
    max_flux = np.max(flux_array)
    min_flux = np.min(flux_array)
    flux_range = max_flux - min_flux #gets brightest and faintest flux in this band and finds range 

    for flux, sxx, syy in zip(flux_array, shape_xx_arr, shape_yy_arr): #loop over each galaxy finding its flux and shape moments
        r_total = np.sqrt(np.abs(sxx + syy)) # estimates the total size of the galaxy from shape moments and the sum is the trace of the moment tensor and represents the spatial exent of light

        # Normalise flux between 0 and 1 (Faintest and Brightest) and doesnt divide by zero and get the same flux which was happening before
        flux_norm = (flux - min_flux) / flux_range if flux_range != 0 else 0.5

        # r_80 scales with size, r_20 scales with brightness
        # These use different relationships so the ratio actually varies
        r_80 = r_total * (0.5 + 0.5 * flux_norm)   # scales between 0.5 and 1 depending on the brightness and bright galaxies will get a larger r_80 since the want to be more extended 
        r_20 = r_total * (0.1 + 0.3 * flux_norm)   # scales between 0.1 and 0.4 

        r_80_list.append(max(r_80, 1e-6))
        r_20_list.append(max(r_20, 1e-6)) #appends the rad to its list enforcing a very value so that we dont get 0 

    return np.array(r_80_list), np.array(r_20_list) # returns list so they be used in the main loop 


columns_to_use = [
    'objectId',
    'g_cModelFlux', 'r_cModelFlux', 'i_cModelFlux', 'z_cModelFlux',
    'y_cModelFlux', 'u_cModelFlux', 'shape_xx', 'shape_yy'
] #We want to extract from these columns 

galaxies = []

reader = csv.DictReader(content.splitlines()) #reads each line where the column names are
for row in reader:
    try: # doesn't skip any line
        galaxy = {col: float(row[col]) if col != 'objectId' else row[col]
                  for col in columns_to_use}
        galaxies.append(galaxy) #Appends each galaxy to galaxy list
    except ValueError:
        pass

band_flux_keys = {
    'u': 'u_cModelFlux',
    'g': 'g_cModelFlux',
    'r': 'r_cModelFlux',
    'i': 'i_cModelFlux',
    'z': 'z_cModelFlux',
    'y': 'y_cModelFlux',
} # maps each band letter to column name

shape_xx_arr = np.array([g['shape_xx'] for g in galaxies])
shape_yy_arr = np.array([g['shape_yy'] for g in galaxies]) #finds all shape moments and arrays them and doesn't need to be rebuilt inside every loop iteration

all_data = [] #collects all results from each row #pokemon scalper basically

for band, flux_key in band_flux_keys.items():# builds array of all galaxies 

    # Full flux array for this band — with all these bands they should have a concert
    flux_array = np.array([g[flux_key] for g in galaxies]) # all 100 whatever galaxies
    flux_180   = flux_array[::-1] #reverses array because of 180 no scope rust 1v1

    
    r_80_arr, r_20_arr = estimate_radii(flux_array, shape_xx_arr, shape_yy_arr)
        # calls esitmate radii so it returns r_80,20 arrays 
    for idx, galaxy in enumerate(galaxies):
        obj_id   = galaxy['objectId']
        flux_val = galaxy[flux_key] # galaxy flux in current band

        # eheheh Lois I'm applying the CAS parameters to E+A's ehehehehe
        C = calculateC(r_80_arr[idx], r_20_arr[idx]) #computes C once per galaxy per band

        # The if else statements after them is so they don't divide by zero cause we keep it 100
        A = np.abs(flux_val - flux_180[idx]) / (np.abs(flux_val) + np.abs(flux_180[idx])) if (flux_val + flux_180[idx]) != 0 else 0.0 #compares flux to mirror position
        filterSize = max(3, int(round(0.3 * r_80_arr[idx])))
        if filterSize % 2 == 0:
            filterSize += 1
        I_S = ndimage.uniform_filter(flux_array, size=filterSize) #blurs the full flux array
        
        S = (np.abs(flux_val - I_S[idx]) / np.abs(flux_val)
             if flux_val != 0 else 0.0) # measures how this galaxy's flux differs from blurred value
        # The if else statements after them is so they don't divide by zero cause we keep it 100

        all_data.append({ # appends data
            'index':    idx + 1,
            'objectId': obj_id,
            'band':     band,
            'C':        round(float(C), 4),
            'A':        round(float(A), 4),
            'S':        round(float(S), 4),
        })

script_folder = Path(__file__).parent #gets folder and saves the csv in it (From API PULL project)

dataFrame = pd.DataFrame(all_data)

timestamp = datetime.now().strftime("%Y%m%d_%H%M%S") #unique name for each saved file depending on computers time and date
file_name = f"cas_results_{timestamp}.csv" # Easy for more files and data sets and because I wanted it
csv_path  = script_folder / file_name

dataFrame.to_csv(csv_path, index=False) #saves data to CSV

latest_file_path = script_folder / "latest_file.txt" #This is cool it tells you what the most recent file is
with open(latest_file_path, "w") as f:
    f.write(file_name)

os.startfile(csv_path)

print("Saved to", csv_path) # Shows where it is

#coulve made it so it shows the result in the terminal but why would I ever do tha
