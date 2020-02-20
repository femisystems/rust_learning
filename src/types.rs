use std::mem::size_of_val;

pub fn primitive_types() {
  /*
  -----------------------------------------

  RUST CHARS

  -----------------------------------------
  */
  let first_char:&str = "c"; // 16 bytes
  let second_char:char = 'b'; // 4 bytes
  /*
      let third_char_a:char = "b";
      this is a tye mis-match because double quotes imply &str type
      which uses 16 bytes in memory --- maddd ohhhhh
      However, if a single-quoted string is not typed, the compiler
      infers it as a char and subsequently uses 4 bytes
  */
  let third_char_b = 'g'; // 4 bytes
  let fourth_char = "t"; // 16 bytes

  println!("first_char \"{}\" uses {} bytes in memory", first_char, size_of_val(&first_char));
  println!("second_char \"{}\" uses {} bytes in memory", second_char, size_of_val(&second_char));
  println!("third_char_b \"{}\" uses {} bytes in memory", third_char_b, size_of_val(&third_char_b));
  println!("fourth_char \"{}\" uses {} bytes in memory", fourth_char, size_of_val(&fourth_char));

  /*
  -----------------------------------------

  RUST NUM

  -----------------------------------------
  */
  let num_a:u8 = 255;
  let num_b:u16 = 65535;
  let num_c:u32 = 4294967295;
  let num_d:f64 = 1.844674407E19; // equal to u64
  
  println!("num_a - {} byte(s)", size_of_val(&num_a));
  println!("num_b - {} byte(s)", size_of_val(&num_b));
  println!("num_c - {} byte(s)", size_of_val(&num_c));
  println!("num_d - {} byte(s)", size_of_val(&num_d));
}