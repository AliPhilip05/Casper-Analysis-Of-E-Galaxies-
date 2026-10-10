# Function Documentation list

This document will consist of a list of the functions used in this research, how to use them and how to modify the code. The documentation will assume that:

    1. Anybody attempting to make modifications to the code is decently versed in the art of programming.

    2. The programmer attempting to modify the code has a basic understanding of Physics I and Physics II.
    3. The programmer is versed in C/C++ Programming.

These three requirements will ensure that regardless of the future changing or refactoring of the code so that the program will function regardless of the language that it is written in.

For those inexperienced with Rust, I will provide basic clarifications for the first function and then proceed to use less detailed documentation after.

`/ Function to calculate the Concentration

fn calculate_c(r_80:f64, r_20:f64) -> f64 {
if r_20 == 0 {
0.0
} else {
let result:f64 = 5.0 * (r_80 / r_20).log10();
}
result}`

`fn calculate_c`

This is the definition for a function called calculate_c that calculates the Concentration

`(r_80:f64, r_20:f64)`

These are the parameters for the function. r_80 and r_20 are both parameter names that are both floating point integers that can contain 2^64 bits of information.

`-> f64`

This is the return type of the function. It simply states that this function that takes in two floating type variables will return a floating type variable. Note that a function can return a variable of any type.

`if r_20 == 0 {
        0.0
    }`

This will automatically return the floating point 0.0 should r_20 = 0. Note that there is a lack of a return keyword in this code. This is because Rust will automatically return the data type of the return type if it is the last line of code in the block. If there was another piece of code such as:

`if r_20 == 0 {
        0.0
	println!("hello");
    }`

The code would not return 0.0 and you would be presented with an error.

`else {
       let result:f64 = 5.0 * (r_80 / r_20).log10();
    }
    result`

This piece of the code simply performs a calculation. It sets the result of type floating point 64 to be equal to the parameters that we took in with some operations done to it. It then returns results. Refer back to the previous piece of code if the return argument is unclear.
