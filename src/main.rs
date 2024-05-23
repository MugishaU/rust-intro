fn main() {
  // Statement block
  {
    let number_1 = 26;
    let number_2 = 31;
    let sum = number_1 + number_2;
    println!("{sum}");
  }

  // Expression block
  let sum = {
    let number_1 = 11;
    let number_2 = 31;
    number_1 + number_2
  };

  let number = 10;

{
  println!("{number}"); // Prints "10"

  let number = 22;
    println!("{number}"); // Prints "22"
} // Our second declaration of `number` is dropped from memory here.
  // It is now considered out-of-scope.

println!("{number}"); // Prints "10"
}
