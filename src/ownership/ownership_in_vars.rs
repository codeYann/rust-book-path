// Ownership rules:
/*
* Each value in Rust has an owner
* There can only be one owner at time
* When the owner goes out of scope, the value will be dropped. Calling Drop() method
* */

pub fn _vars_and_scopes() {
    // Rust uses RAII approach in scopes to deal with free memory
    let message = "ola";
    {
        // When this scope goes out str_var is dropped but drop method.
        // Which means that str_var is no longer usable.
        let str_var = String::from(" mundo!");
        println!("{}{}", message, str_var);
    }
}

pub fn _vars_in_heap_memory() {
    // When we perform a assign to a new variable rust moves the owner
    // of this first var to the second one. Which makes the first vars unusable
    // But, actually, this is only for values stored in heap memory.
    let s1 = "ola".to_string();
    let s2 = s1;

    println!("{}", s2);
}

pub fn _vars_in_stack_memory() {
    let mut s1 = 32;
    let s2 = s1;
    let s3 = s2;
    println!("{}, {}, {}", s1, s2, s3);

    s1 = 35;
    println!("{}, {}, {}", s1, s2, s3);
    // At the end of this line we can see that rust perform a copy of values
    // In other words, we can easly copy values that stored in stack memory
    // and don't worry about ownership
    // This happens because make copy in stack memory is no verbose.
}
