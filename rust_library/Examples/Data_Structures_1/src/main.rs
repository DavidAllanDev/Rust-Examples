#![allow(dead_code)]
#![allow(unused_variables)]
mod pm;

fn arrays()
{
  let mut a/*:[i32;5]*/ = [1,2,3,4,5];
  
  println!("a has {} elements, first is {}", a.len(), a[0]);
  a[0] = 321;
  println!("first value of a is {}", a[0]);

  assert_eq!(a, [321,2,3,4,5]);

  if a != [1,2,3,5,6] // size must match
  { 
    println!("arrays not equal!");
  }

  // fill an array with 1s
  let b = [1u16; 10]; // try changing to 5
  for i in 0..b.len()
  {
    println!("{}", b[i]);
  }

  // just print the entire arary
  println!("{:?}", b);
  println!("b took up {} bytes", mem::size_of_val(&b));

  // multidimensional array
  let mtx:[[f32; 3]; 2] = 
  [
    [1.0, 0.0, 0.0],
    [0.0, 2.0, 0.0]
  ];
  println!("{:?}", mtx);

  // print all the diagonal values
  for i in 0..mtx.len()
  {
    for j in 0..mtx[i].len()
    {
      if i == j 
      { 
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
      }
    }
  }
}


fn vectors()
{
  /*
  let mut a = Vec::new();

  a.push(1);
  a.push(2);
  a.push(3);
  */

  let mut a = vec![1,2,3]; // [1;10]
  println!("a = {:?}", a);

  let idx/*:i32*/ = 0; // will not work with :i32
                       // you need usize
  println!("a[0] = {}", a[idx]);

  // unsafe access
  //println!("a[5] = {}", a[5]);

  match a.get(5)
  {
    Some(x) => println!("a[5] = {}", x),
    None => println!("error, no such element")
  }

  // iterate
  for x in &a { println!("{}", x); }

  // adding/removing
  a.push(44);
  println!("{:?}", a);

  let last_elem = a.pop(); // can easily yield nothing
  println!("last elem is {:?}, a = {:?}", last_elem, a);

  // explain why this doesn't work
  //let Some(last_value) = a.pop();

  // print the elements in reverse order
  while let Some(x) = a.pop()
  {
    println!("{}", x);
  }
}

struct A; // does absolutely nothing

struct Point
{
  x: f64,
  y: f64
}

struct Line
{
  start: Point,
  end: Point
}


impl Line
{
  fn len(&self) -> f64
  {
    let dx = self.start.x - self.end.x; // Ctrl+D
    let dy = self.start.y - self.end.y;
    return (dx*dx+dy*dy).sqrt();
  }
}




fn structures()
{
  let p = Point { x: 3.0, y: 4.0 };
  println!("point p is at ({},{})", p.x, p.y);

  let p2 = Point { x: 5.0, y: 10.0 };
  let myline = Line { start: p, end: p2 };
  

  // destructuring

  // member functions
  println!("line length is {}", myline.len());
}

fn options()
{
  let x = 3.0;
  let y = 0.0;

  let result:Option<f64> =
    if y != 0.0 { Some(x/y) } else { None };

  match result {
    Some(z) => println!("{}/{}={}", x, y, z),
    None => println!("cannot divide {} by {}", x, y)
  }

  if let Some(z) = result { println!("result = {}", z); }

  // while let
}

enum Color
{
  Red, // unit-like struct
  Green,
  Blue,
  RgbColor(u8,u8,u8), // tuple struct
  CmykColor{cyan:u8,magenta:u8,yellow:u8,black:u8},
}

fn enums()
{
  let c = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};
    //Color::RgbColor(0,0,0);

  match c
  {
    Color::Red => println!("r"),
    Color::Green => println!("g"),
    Color::Blue => println!("b"),

    Color::RgbColor(0,0,0) 
    | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} 
      => println!("black"),

    Color::RgbColor(r,g,b) => println!("rgb({},{},{}", r, g, b),
    _ => ()
  }
}

struct Point<T>
{
  x: T,
  y: T
}

struct Line<T>
{
  start: Point<T>,
  end: Point<T>
}

fn main() {
  //options();
  //enums();
  //structures();
  //arrays();
  //vectors();
}