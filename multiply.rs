//벡터 형식의 숫자 자리 맞추기
fn normalize(a: &mut Vec<i32>){
    let mut num=0;
    loop{
        if a[num]>10{
            if num==a.len()-1 {a.push(0);}
            a[num+1]+=a[num]/10;
            a[num]%=10;
        }
        num += 1;
        if num==a.len() {break;}
    }
    for i in (0..a.len()).rev(){
        if a[i]==0 {a.pop();}
        else {break;}
    }
}

//벡터 형식의 숫자 출력
fn print(a: &Vec<i32>){
    for num in a.iter().rev(){
        print!("{}",num);
    }
    println!();
}