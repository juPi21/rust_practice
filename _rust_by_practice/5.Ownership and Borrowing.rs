//Ownership
//1.1
fn main() {
  // Use as many approaches as you can to make it work
  let x = String::from("Hello world");
  let y = &x;
  println!("{}, {}",x, y);
}
//1.2
fn main() {
  let x = "hello, world";
  let y = x;
  println!("{},{}",x,y);
}
//2.
// Don't modify code in main!
fn main() {
  let s1 = String::from("hello, world");
  let s2 = take_ownership(s1);

  println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
  println!("{}", s);
  s
}
//3.
fn main() {
  let s = give_ownership();
  println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
  let s = String::from("hello, world");
  s
}
//4
fn main() {
  let s = String::from("hello, world");
  print_str(&s);
  println!("{}", s);
}
fn print_str(s: &String)  {
  println!("{}",s)
}
//5.
fn main() {
  let x = (1, 2, (), "hello");
  let y = x;
  println!("{:?}, {:?}", x, y);
}
//6.
fn main() {
  let s = String::from("hello, ");
  
  // modify this line only !
  let mut s1 = s;

  s1.push_str("world")
}
//7.
fn main() {
  let x = Box::new(5);
  
  let mut y = Box::new(3);       // implement this line, dont change other lines!
  
  *y = 4;
  
  assert_eq!(*x, 5);
}
//8.
fn main() {
  let t = (String::from("hello"), String::from("world"));

  let _s = t.0;

  // modify this line only, don't use `_s`
  println!("{:?}", t.1);
}
//9.
fn main() {
  let t = (String::from("hello"), String::from("world"));

  // fill the blanks
  let (s1, s2) = t.clone();

  println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
//Borrowing
//1.
fn main() {
  let x = 5;
  // fill the blank
  let p = &x;

  println!("the memory address of x is {:p}", p); // one possible output: 0x16fa3ac84
}
//2.
fn main() {
  let x = 5;
  let y = &x;

  // modify this line only
  assert_eq!(5, *y);
}
//3.
fn main() {
  let mut s = String::from("hello, ");

  borrow_object(&s)
}

fn borrow_object(s: &String) {}
//4.
fn main() {
  let mut s = String::from("hello, ");

  push_str(&mut s)
}

fn push_str(s: &mut String) {
  s.push_str("world")
}
//5.
fn main() {
  let mut s = String::from("hello, ");

  // fill the blank to make it work
  let p = &mut s;
  
  p.push_str("world");
}
//6.
fn main() {
  let c = '中';

  let r1 = &c;
  // fill the blank，dont change other code
  let ref r2 = c;

  assert_eq!(*r1, *r2);
  
  // check the equality of the two address strings
  assert_eq!(get_addr(r1),get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
  format!("{:p}", r)
}
//7.
fn main() {
  let s = String::from("hello");

  let r1 = &s;
  let r2 = &s;

  println!("{}, {}", r1, r2);
}
//8.
fn main() {
  //fix error by modifying this line
  let mut s = String::from("hello, ");

  borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}
//9.
fn main() {
  let mut s = String::from("hello, ");

  borrow_object(&s);
  
  s.push_str("world");
}

fn borrow_object(s: &String) {}

//10.
fn main() {
  let mut s = String::from("hello, ");

  let r1 = &mut s;
  r1.push_str("world");
  let r2 = &mut s;
  r2.push_str("!");
  
  // println!("{}",r1);
}
//11.
fn main() {
  let mut s = String::from("hello, ");

  let r1 = &mut s;
  let r2 = &mut s;

  // add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
  // you can't use r1 and r2 at the same time
  r1.push_str("world");
}