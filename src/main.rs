// Function to calculate the Concentration 
fn calculate_c(r_80:f64, r_20:f64) -> f64 {
    if r_20 == 0 {
        0.0
    } else {
       let result:f64 = 5.0 * (r_80 / r_20).log10();
    }
    result
}

// Function to calculate the Asymmetry 
fn calculate_a(i; f32, i_180: f32, a_bkg: f32) -> f32 {
   //TODO: Fill in the body of the code
   //numerator: f32 = ?
   //
}

// Function to calculate the  Smoothness 
fn calculate_s() // TODO: Inputs and outputs
                 // Function Definition 

fn estimate_radii(//TODO: fill out function) -> ??? {
                  //TODO: Fill in body definition 
                  //}

fn load_galaxies(//TODO: Fill in requirements) -> ??? {
                 //TODO: Refactor the code to make this a function 
                 //rather than a body
                 //}

fn main() {
    println!("Hello, world!");
}

//TODO: Add in test cases for each function
//
