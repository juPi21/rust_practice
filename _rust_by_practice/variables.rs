//1. A variable can be used only if it has been initialized.
// Fix the error below with least amount of modification to the code
fn main() {
  let x: i32 = 5; // Uninitialized but used, ERROR !
  let y: i32; // Uninitialized but also unused, only a Warning !

  assert_eq!(x, 5);
  println!("Success!");
}
//2. Use mut to mark a variable as mutable.
// Fill the blanks in the code to make it compile
fn main() {
  let mut x: i32 = 1;
  x += 2; 
  
  assert_eq!(x, 3);
  println!("Success!");
}
//Scope
//3.
// Fix the error below with least amount of modification
fn main() {
  let x: i32 = 10;
  let y: i32 = 5;
  {
      
      println!("The value of x is {} and value of y is {}", x, y);
  }
  println!("The value of x is {} and value of y is {}", x, y); 
}
//4.
// Fix the error with the use of define_x
fn main() {
  let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> &'static str {
    let x = "hello";
    return x
}
//Shadowing
//5.
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
  let x: i32 = 5;
  {
      let x = 12;
      assert_eq!(x, 12);
  }

  assert_eq!(x, 5);

  let x = 42;
  println!("{}", x); // Prints "42".
}
//6.

// Remove a line in the code to make it compile
fn main() {
  let mut x: i32 = 1;
  x = 7;
  // Shadowing and re-binding
  let x = x; 


  let y = 4;
  // Shadowing
  let y = "I can also be bound to text!"; 

  println!("Success!");
}
//Unused variables
//7.

fn main() {
  let x = 1;
  println!("{}", x);
}
//Destructuring
//8.
// Fix the error below with least amount of modification
fn main() {
  let (mut x, y) = (1, 2);
  x += 2;

  assert_eq!(x, 3);
  assert_eq!(y, 2);

  println!("Success!");
}
//Destructuring assignments
//9.
fn main() {
  let (x, y);
  (x,..) = (3, 4);
  [.., y] = [1, 2];
  // Fill the blank to make the code work
  assert_eq!([x,y], [3, 2]);

  println!("Success!");
} 