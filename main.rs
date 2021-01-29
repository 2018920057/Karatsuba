use std::cmp;

fn main() {
  let a : Vec<i32> = vec![1,5,6,1];
  let mut b : Vec<i32> = vec![3,19];
  normalize(&mut b);
  println!("{:?}",b);
  let c : Vec<i32> = karatsuba(&a,&b);
  println!("a:{:?}, b:{:?}, a*b:{:?}",print(&a),print(&b),print(&c));
}

//a += b*(10^k)
fn addTo(a : &mut Vec<i32>, b : & Vec<i32>, k : usize){
  let afterlength = b.len()+k;
  let mut i = k;
  //b*(10^k)가 a보다 클 경우
  while a.len()<afterlength{a.push(0);}
  //더하기
  while i<afterlength{
    a[i] += b[i-k];
    print!("{}",a[i]);
    i+=1;
  }
  normalize(a);
}

//a -= b 
fn subFrom(a : &mut Vec<i32>, b : & Vec<i32>){
  let blength = b.len();
  if blength>a.len() {panic!("error");}
  for i in 0..blength {
    a[i]-=b[i];
  }
  normalize(a);
}

//a*b
fn karatsuba(a : & Vec<i32>, b : & Vec<i32>) -> Vec<i32>{
  let alength = a.len();
  let blength = b.len();
  //a가 b보다 짧을 경우 a와 b를 바꾼다.
  if(alength<blength) {return karatsuba(b,a);}
    
  let mut ret : Vec<i32> = Vec::new();
  //기저 사례: a나 b가 비어있다면
  if(alength*blength==0) {return ret;}
  //a와 b를 절반과 나머지 자리로 분리
  //a = a1*(10^half)+a0
  let ahalf = alength/2;
  let mut a0 = a[..ahalf].to_vec();
  let a1 = a[ahalf..].to_vec();
  let bhalf = cmp::min(ahalf,blength);
  let mut b0 = b[..bhalf].to_vec();
  let b1 = b[bhalf..].to_vec();
  //z2 = a1*b1
  let z2 = karatsuba(&a1,&b1);
  //z0 = a0*b0
  let z0 = karatsuba(&a0,&b0);
  //z1 = (a0+a1*b0+b1)-z0-z2
  addTo(&mut a0, &a1, 0);
  addTo(&mut b0, &b1, 0);
  let mut z1 = karatsuba(&a0,&b0);
  subFrom(&mut z1, &z0);
  subFrom(&mut z1, &z2);
  //ret = z0 + z1*(10^ahalf) + z2*(10^2half)
  addTo(&mut ret, &z0, 0);
  addTo(&mut ret, &z1, ahalf);
  addTo(&mut ret, &z2, 2*ahalf);
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