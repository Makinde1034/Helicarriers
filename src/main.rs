// The modular multiplicative inverse of an interge a, is another integer x such that the product of a and x is congruence to 1 with respect to modulus m. 

// Two integers a and b are cruguent [ a===b (modulus m). ] when a - b is didvisible by m and diving a and b by m have thesame remainder. 

// The euclidean algorithm computes the greatest common divisor of two numbers and it follows a series of iterative steps. The output of each step is used as an input for the next one.



// function declaration
fn main(){
    // Println! is used to get an output. Similar to cosole.log()
    // function modular_inverse is called and the integer whose multiplicative inverse is to be determined and the modulus are passed in as arguments.
    println!("{}",modular_inverse(3,26))
}


// Function declaration which takes in two parameters of type isize (types whose size depends on the size of the pointer of the underlined machine) and returns type isize.
// a represents an integer whose multiplicative inverse is to be computed.
// modulus represents an integer whose value is greater than 1

fn modular_inverse(a : isize, modulus : isize) ->isize{
    // Declaring a mutable variable set equal to a tuple (stores a bunch of variables in one collection) that holds "a" and "modulus" .
    let mut num_mod = (modulus,a);
    
    // Another mutable variable set equal to a tuple that counts the step of the algorithm. 
    let mut counter = (0,1);

    // While loop allows the execution of code as long as a codition is true useful in this case to perform an iteration.
    // here, the value of "a" is accessed using its index and a condition is specified that the block should run as long as integer "a" is not equal to zero

    while num_mod.1 !=0 {
        // for each iteration, a new tuple is generated for num_mod and counter :
        // 1). integer in index position 1 in counter tuple is moved to index position 0.
        // 2). integer in index position 0 in tuple counter is subtracted from the division of  "modulus" and "a" mutiplied by integer in index position 1 in tuple counter

        counter = (counter.1, counter.0 -  (num_mod.0/num_mod.1)  * counter.1 );

        // For tuple num_mod :
        // 1). integer in index position 1 in num_mod tuple is moved to index position 0.
        // 2). The remainder when "modulus" divided by "a" is obtained using the modulus operator (%). 
        
        num_mod = (num_mod.1, num_mod.0 % num_mod.1);

    }

    // while loop to increament the integer in position 0 of counter tuple  by the value of "modulus" argument if it's present value is lesser than 0 
    while counter.0 < 0 {
        counter.0 += modulus 
    }

    counter.0
    


}