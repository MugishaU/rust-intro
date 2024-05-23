fn main() {
  fn double(num: u128) -> u128 {
    num * 2
  }

  // let int: i32 = 32;
  let big_int = 105;
  // let float = 1.2;

  let input = big_int;

  let output = double(big_int);
  // let doubled_outcome = outcome;

  println!("{input} doubled is {output}");
}
