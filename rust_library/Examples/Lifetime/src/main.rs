#![allow(dead_code)]
#![allow(unused_variables)]

fn ownership()
{
  let v = vec![3,2,1]; // v owns what it is bound to

  // v is on the stack, data is on the heap

  //let v2 = v;
  // this makes a copy of that pointer for v2
  // two pointers to same content
  // introduces a race & violates rust's safety guarantees


  // there is only one highlander! binding to a resource

  println!("{:?}", v);

  //let foo = |v:Vec<i32>| ();
  //foo(v);

  println!("{:?}", v);

  // ownership transfer = move

  let u = 1; // i32 implements a Copy trait
  let u2 = u; // instead of move, we copy

  // question of who deallocates

  // i32 has no pointers to data, copying = full copy
  // if you don't want this, use Box<i32>

  println!("u = {}", u);

  // all primitive types (i32, bool, f64) implement the copy trait

  // hand ownership back

  let print_vector = |x:Vec<i32>| -> Vec<i32>
  {
    println!("{:?}", x);
    x
  };

  let vv = print_vector(v);
  println!("{}", vv[0]);
}

fn borrowing()
{
  let print_vector = |x:&Vec<i32>| // take a reference
  {
    println!("x[0] = {}", x[0]);

    // borrowing = do not deallocate

    // references are immutable
    //x.push(4);
  };

  let v = vec![3,2,1];
  print_vector(&v);
  println!("v[0] = {}", v[0]);

  // mutable references
  let mut a = 40;
  { // try removing braces to get an error
    let b = &mut a;
    *b += 2; // use * to access the content of a references
  }
  println!("a = {}", a);

  // why?

  // rules for protecting against data races:
  // 1+ references to a resource
  // exactly ONE mutable ref

  let mut z = vec![3,2,1];

  for i in &z
  {
    println!("z = {}", i);
    // will not work
    //z.push(5);
  }
}

// lifetime

fn bar<'a>(x: &'a mut i32) -> &mut i32 // lifetime elision
{
  x
}

fn lifetime()
{
  // 'static = variable lives for the lifetime of the program
  let hello:&'static str = "hello, Rust!";

  // this will not work
  let mut v = vec![100,200];
  
  { // add later
  let x = &v[0];
  println!("{}", x);
  }
  v.push(4);
  let x = &v[0];
  println!("{}", x);

  // will not work
  /*
  let x;
  {
    let y = &5;
    x = y;
  } // y dies here
  println!("x = {}", *x);
  */
  
   
}

fn main() {
  //ownership();
  //borrowing();
  lifetime();
}