fn main() {
  let mut a : Vec<i32> = vec![1,2,3,5,2,5,6];
  let mut b : Vec<i32> = vec![3,1,4,5,1,-5,19];
  normalize(&mut b);
  println!("{:?}",b);
  let c : Vec<i32> = karatsuba(&a,&b);
  
  println!("a:{:?}, b:{:?}, a*b:{:?}",print(&a),print(&b),print(&c));
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

//벡터 형식의 숫자 자리 맞추기
fn normalize(a: &mut Vec<i32>){
  let length :usize = a.len();
  a.push(0);
  //자릿수 올림 처리
  for i in 0..length{
    if(a[i]<0){
      let borrow:i32 = (a[i].abs()+9)/10;
      a[i+1] -= borrow;
      a[i] += borrow*10;
    }else{
      a[i+1] += a[i] / 10;
      a[i] %= 10;
    }
  }
  while a.len()>1&&a[a.len()-1]==0 {a.pop();}
}

//벡터 형식의 숫자 출력
fn print(a: &Vec<i32>) -> String{
  a.iter().rev().map(ToString::to_string).collect()
}