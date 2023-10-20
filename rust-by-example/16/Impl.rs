fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
  numbers
      .iter()
      .filter(|x| x > &&0)
      .map(|x| x * 2)
}

fn d_ps(numbers: &Vec<i32>) -> impl Iterator<Item = i32> + '_ {
  numbers
      .iter()
      .filter(|x| x > &&0)
      .map(|x| x * 2)
}

fn main() {
  let singles = vec![-3, -2, 2, 3];
  let doubles = double_positives(&singles);
  let ds = d_ps(&singles);
  assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
  assert_eq!(ds.collect::<Vec<i32>>(), vec![4, 2]);
}