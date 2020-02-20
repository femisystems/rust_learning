use std::f64::consts::PI;

pub fn operators_fn() {
  /*
  -----------------------------------------

  BASIC ARITHMETIC OPERATION

  -----------------------------------------
  */
  let a = 5;
  let b = 6;
  
  let mut c = a + b;
  println!("c = {}", c);
  
  c += 5;
  println!("c = {}", c);


  /*
  -----------------------------------------

  ADVANCED ARITHMETIC OPERATION

  -----------------------------------------
  */
  let cube_c = i32::pow(c, 3);
  println!("cube_c = {}", cube_c);

  // cube of a floating point number
  let num = 2.7;

  // ::powi is used when the power is an integer
  let cube_num_int = f64::powi(num, 3);
  println!("cube_num_int = {}", cube_num_int);

  // ::powf is used when power is a floating point number
  let cube_num_float = f64::powf(num, PI); // or
  let cube_num_float_b = f64::powf(num, 4.6);
  println!("cube_num_float = {}", cube_num_float);
  println!("cube_num_float_b = {}", cube_num_float_b);

  /*
  -----------------------------------------

  LOGICAL OPERATION

  -----------------------------------------
  */
  let zebra = "zebra";
  let goat = "goat";
  let zebra_not_goat = zebra == goat;
  println!("{}", zebra_not_goat);

  let num_a = 5;
  let num_b = 6;
  let is_b_greater = num_b > num_a;
  println!("{}", is_b_greater);  
}
