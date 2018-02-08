#![allow(dead_code)]
#![allow(unused_variables)]

// functions
fn print_value(x:i32)
{
  println!("value = {}", x);
}

fn increase(x: &mut i32) // start with i32
{
  *x += 1;
}

fn product(x: i32, y: i32) -> i32 // return value
{
  let z = x * y;
  z // no semicolons
}

fn functions()
{
  print_value(123);

  let mut z = 1;
  increase(&mut z); // lend z
  println!("z = {}", z);

  let a = 3;
  let b = 5;
  let p = product(a,b);
  println!("{} * {} = {}", a, b, p);
}

fn say_hello() { println!("hello"); }

fn closures()
{
  let sh = say_hello;
  sh();

  let plus_one = |x:i32| -> i32 { x + 1 };
  let a = 6;
  println!("{} + 1 = {}", a, plus_one(a));

  let mut two = 2;
  {
    let plus_two = |x|
    {
      let mut z = x;
      z += two;
      z
    };
    println!("{} + 2 = {}", 3, plus_two(3));
  }

  let borrow_two = &mut two;

  // T: by value
  // T&
  // &mut &
  let plus_three = |?mut x:i32| x += 3;

  let mut f = 12;
  plus_three(f);
  println!("f = {}", f);
}

// higher-order functions
fn is_even(x: u32) -> bool
{
  x % 2 == 0
}

fn hof()
{
  // functions that take functions

  // sum of all even squares <= 500

  let limit = 500;
  let mut sum = 0;
  for i in 0..
  {
    let isq = i*i;

    if isq > limit { break; }
    else if is_even(isq) { sum += isq; }
  }

  println!("loop sum = {}", sum);

  let sum2 =
    (0..).map(|x| x*x)
         .take_while(|&x| x < limit)
         .filter(|x| is_even(*x))
         .fold(0, |sum,x| sum + x);
  println!("hof sum = {}", sum2);
}

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
    let dx = self.start.x - self.end.x;
    let dy = self.start.y - self.end.y;
    (dx*dx+dy*dy).sqrt()
  }
}

fn methods()
{
  let p = Point { x: 3.0, y: 4.0 };
  let p2 = Point { x: 5.0, y: 10.0 };
  let myline = Line { start: p, end: p2 };

  println!("length = {}", myline.len());
}

trait Animal
{
  // static: called as Animal::create()
  // returns the type of the implementor
  fn create(name: &'static str) -> Self;
  
  fn name(&self) -> &'static str;

  fn talk(&self)
  {
    println!("{} cannot talk", self.name());
  }
}

struct Human
{
  name: &'static str
}

impl Animal for Human
{
  fn create(name:&'static str) -> Human
  {
    Human{name: name}
  }

  fn name(&self) -> &'static str
  { 
    self.name
  }

  fn talk(&self)
  {
    println!("hello, my name is {}", self.name());
  }
}

struct Cat
{
  name: &'static str
}

impl Animal for Cat
{
  fn create(name:&'static str) -> Cat
  {
    Cat{name: name}
  }

  fn name(&self) -> &'static str
  { 
    self.name
  }

  fn talk(&self)
  {
    println!("{} says meow", self.name());
  }
}

fn traits()
{
  // println some custom type
  //let h = Human{name:"John"};
  let h:Human = Animal::create("John"); // type annotation mandatory!
  h.talk();

  //let m = Cat{name:"Misty"};
  let m:Cat = Animal::create("Misty");
  m.talk();

  // different construction now
  // select invocation implementation based on receiver

  // extend other types
  let a = vec![1,2,3];
  println!("sum = {}", a.sum());

  // todo: derive, op overloading, and other stuff from abc
}

fn main() 
{
  
  //functions();
  //methods();
  //closures();
  //hof();
  //traits();
 
}