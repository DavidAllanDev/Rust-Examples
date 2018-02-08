#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
use std::mem;



// try lowercase naming first
// NEED type decoration
const MEANING_OF_LIFE:u8 = 42; // lifetime = entire program
                               // inlined!
                               // no fixed address in memory

// just an ordinary global
static mut Z:i32 = 123; // thread unsafe!

fn operators()
{
  // arithmetic

  let mut a = 2+3*4; // +-/*
  println!("a = {}", a);

  a = a + 1; // no ++ or -- operators
  a -= 2; // +=, *=, /=, %=

  println!("remainder of {} / {} = {}", a, 3, (a%3));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed = {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

  // bitwise
  let c = 1 | 2; // ORing 01 and 10, resulting in 11 (3_10)
  println!("1 | 2 = {}, inverted c = {}", c, !c);
  //let d = 1.0 | 2.0; // will not compile

  let two_to_10 = 1 << 10;
  println!("2 to the power of 10 = {}", two_to_10);

  // logical
  let pi_less_4 = std::f64::consts::PI < 4.0; // true
  // < > <= >=
  // ==
  let x = 5;
  let x_is_5 = (x == 5);
}

fn constants()
{
  println!("the meaning of life is {}", MEANING_OF_LIFE); // inlined!

  // cannot change constant
  // MEANING_OF_LIFE = 10000000;

  unsafe { // compiler cannot verify this code, but it's actually safe
    println!("Z = {} before change", Z);
    Z = 555;
    println!("Z = {} after change", Z);
  }
}

fn vars_and_types() 
{
  // variable declaration scoped to current function
  let a:u8 = 123; // unsigned, immutable
  println!("a = {}", a);

  // this won't work
  // a = 456;

  let mut b:i8 = 0; // mutable
  println!("b = {} before", b);
  b = 42;
  println!("b = {} after", b);

  b = 456; // out of range inspection!
  println!("b = {} oops...", b);

  // avoid type decoration
  let mut c = 123456789; // it's an i32!
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -1;
  println!("c = {} after modification", c);

  let z:isize = 123; // usize
  let size_of_z = mem::size_of_val(&z);
  println!("z = {}, takes up {} bytes, {}-bit OS", 
    z, size_of_z, size_of_z * 8);

  let d:char = 'x'; // no type deco, single 32-bit unicode character
  println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

  let e:f32 = 2.5;
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  let f:f64 = 2.5; // default float type
  println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

  // boolean variables
  let g = false; // true, g is of type boolean
  println!("g = {}, size = {} byte(s)", g, mem::size_of_val(&g));
}

fn scope_and_shadowing()
{
  let a = 123;

  { 
    // block defines a scope
    let b = 456;
    println!("inside, b = {}", b);

    // define another 'a'
    let a = 1000; // shadows outside 'a'
    println!("inside, a = {}", a);
  }

  println!("outside, a = {}", a);

  // cannot print b
  //println!("outside b = {}", b);
}

fn main()
{
  // won't work, a's scope is the function vars_and_types!
  //println!("{}", a);

  sh::stack_and_heap();
  //operators();
  //scope_and_shadowing();
  //vars_and_types();
  //constants();
}