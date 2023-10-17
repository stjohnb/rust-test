fn main() {
  let vec1 = vec![1, 2, 3];

  // `iter()` for vecs yields `&i32`.
  // let mut iter = vec1.iter();

  // `iter()` for vecs yields `&i32`, and we want to reference one of its
  // items, so we have to destructure `&&i32` to `i32`
  println!("Find 2 in vec1: {:?}", vec1.iter().find(|&x| x == 2));

}