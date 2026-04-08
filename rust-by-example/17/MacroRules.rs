// This is a simple macro named `say_hello`.
macro_rules! say_hello {
  // `($name:expr)` indicates that the macro takes one expression argument.
  ($name:expr) => {
      // The macro will expand into the contents of this block.
      println!("Hello, {}!", $name)
  };
}

fn main() {
  // This call will expand into `println!("Hello")`
  say_hello!(String::from("Peter"))
}