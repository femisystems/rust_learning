pub fn scope_test() {
  let a = "outer scope value A";

  {
    let b = "inner scope value b";
    println!("Outer A - {} is available within the inner scope", a);
    println!("Inner B - {} is available within inner scope only", b);
  }

  println!("Outer A - {} is available within the outer scope", a);
}

pub fn shadow_test() {
  let a:u8 = 200;
  println!("Outer A - {} before shadowing", a);

  {
    // shadowed "a"
    let a = "juggernaut";
    println!("Inner A - {} after shadowing", a);
  }

  println!("Outer A - {} outside shadowing scope", a);
}