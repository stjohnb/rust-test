fn main() {
  // Variables can be type annotated.
  #[allow(unused_variables)]
  let logical: bool = true;

  #[allow(unused_variables)]
  let a_float: f64 = 1.0;  // Regular annotation
  #[allow(unused_variables)]
  let an_integer   = 5i32; // Suffix annotation

  // Or a default will be used.
  #[allow(unused_variables)]
  let default_float   = 3.0; // `f64`
  #[allow(unused_variables)]
  let default_integer = 7;   // `i32`

  // A type can also be inferred from context.
  #[allow(unused_variables)]
  let mut inferred_type = 12; // Type i64 is inferred from another line.
  inferred_type = 4294967296i64;

  // A mutable variable's value can be changed.
  #[allow(unused_variables)]
  let mut mutable = 12; // Mutable `i32`
  mutable = 21;

  // Error! The type of a variable can't be changed.
  // mutable = true;

  // Variables can be overwritten with shadowing.
  #[allow(unused_variables)]
  let mutable = true;
}