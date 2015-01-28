// tests live here. run rustc --test tests.rs

#[allow(dead_code)]
// this main function is required even though it never gets called when compiled with --testmain function is required even though it never gets called when compiled with --test
fn main(){}

#[test]
fn test_code(){
  println!("");
}

#[test]
fn more_tests(){
  panic!("This is a failure message");
}
