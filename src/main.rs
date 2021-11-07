///main function that runs the program
fn main() {
    println!("The Modular multiplicative inverse of 3: {}", mod_inv(3, 26)); // finds the modular multiplicative inverse of 3 mod 26
}

///The Extended Euclidean algorithm is an extension to the Euclidean algorithm, and computes, 
///in addition to the greatest common divisor (gcd) of integers a and b, also the coefficients of 
///Bézout's identity, which are integers x and y such that 
///a*x + b*y = gcd(a, b).
///the algorithm is particularly useful when a and b are coprime. With that provision, x is the 
///modular multiplicative inverse of a modulo b, and y is the modular multiplicative inverse of b modulo a.
///(https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
/// ```
///  # Examples
/// ```
/// assert_eq!(mod_inv(3, 26), 9);
/// ```
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) { // state the parameters, type and return type of the function 
    if a == 0 {                                  
        return (b, 0, 1);                       
    }
    let (g, y, x) = egcd(b % a, a);            // recursive call to the function
    (g, x - (b / a) * y, y)                    // return the values of the parameters of the function 
}

///modular multiplicative inverse of an integer a is an integer x such that the product ax 
///is congruent to 1 with respect to the modulus 
///(https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
pub fn mod_inv(a: i64, m: i64) -> i64 { // state the type of a, m and the return value
    let (_g, x, _) = egcd(a, m);        // call the egcd function and store the return values in the variables g, x, and y  
    x % m                              // return the value of x modulo m
}
