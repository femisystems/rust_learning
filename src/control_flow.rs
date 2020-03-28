use std::io;
use std::cmp::Ordering;

pub fn if_conditional() {
  let standard_temperature: f32 = 20.0;
  let mut temperature = String::new();

  io::stdin().read_line(&mut temperature)
    .expect("Failed to read temperature");
  
  let temperature: f32 = temperature.trim().parse()
    .expect("Please enter a number");
  
  match temperature.cmp(&standard_temperature) {
      Ordering::Less => println!("The temperature is cold today"),
      Ordering::Greater => println!("The temperature is warm today"),
      Ordering::Equal => println!("The temperature is balanced today")
  }
}
