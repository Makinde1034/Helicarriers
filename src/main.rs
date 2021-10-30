// The modular multiplicative inverse of an interge a, is another integer x such that the product of a and x is congruence to 1 with respect to modulus m. 

// Two integers a and b are cruguent [ a===b (modulus m). ] when a - b is didvisible by m and diving a and b by m have thesame remainder. 

// The euclidean algorithm computes the greatest common divisor of two numbers and it follows a series of iterative steps. The output of each step is used as an input for the next one.

fn main(){
    println!("Hello worldp"); 
}

// Function declaration which takes in two parameters of type isize (types whose size depends on the size of the pointer of the underlined machine).
fn modular_inverse(a : isize, modulus : isize){
    // Declaring a mutable variable set equal to a tuple (stores a bunch of variables in one collection) that holds a and modulus
    let mut num_mod = (modulus,a)
    
    // Another mutable variable set equal to a tuple that counts the step of the algorithm 
    let mut counter = (0,1);
    
    


}