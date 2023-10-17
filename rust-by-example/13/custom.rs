#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

#[cfg(not(some_condition))]
fn conditional_function() {
    println!("condition not met!");
}


fn main() {
    conditional_function();
}