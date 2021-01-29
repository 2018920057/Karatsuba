fn main() {
  a : mut [i32] = [1,2,3,5,2,5,6];
  b : mut [i32] = [3,1,4,5,1,5,9];
  c : [i32] = karatsuba(&a,&b);
  println!("a:{:?}, b:{:?}, a*b:{:?}",c);
}

//a += b*(10^k)
fn addTo(a : &mut [i32], b : & [i32], k : i32){
  //TODO
}

//a -= b 
fn subFrom(a : &mut [i32], b : & [i32]){
  //TODO
}

//a*b
fn karatsuba(a : & [i32], b : &[i32]) -> [i32]{
  //TODO
}