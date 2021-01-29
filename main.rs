fn main() {
  let mut a : Vec<i32> = vec![1,2,3,5,2,5,6];
  let mut b : Vec<i32> = vec![3,1,4,5,1,5,9];
  let c : Vec<i32> = karatsuba(&a,&b);
  println!("a:{:?}, b:{:?}, a*b:{:?}",a,b,c);
}

//a += b*(10^k)
fn addTo(a : &mut Vec<i32>, b : & Vec<i32>, k : i32){
  //TODO
}

//a -= b 
fn subFrom(a : &mut Vec<i32>, b : & Vec<i32>){
  //TODO
}

//a*b
fn karatsuba(a : & Vec<i32>, b : & Vec<i32>) -> Vec<i32>{
  //TODO
  let mut ret : Vec<i32> = Vec::new();
  ret
}