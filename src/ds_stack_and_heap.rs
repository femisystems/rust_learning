#![allow(dead_code)]

use std::mem;
use std::fmt;

struct Point {
  x: f64,
  y: f64
}

fn origin() -> Point {
  Point {
    x: 0.0,
    y: 0.0
  }
}

/**
 * We implement a way to display/render struct Point.
 * Since Point is not a regular string type, it can't be rendered
 * as such, so we create a function that formats and passes it back
 * to our standard Println! macro
 */
impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}, {}", self.x, self.y)
  }
}

pub fn stack_and_heap() {
  let p1 = origin();
  let p2 = Box::new(origin());
  
  println!("p1 uses a memory size of {}", mem::size_of_val(&p1));
  println!("p2 uses a memory size of {}", mem::size_of_val(&p2));
  println!("The origin is {}", p2);
}