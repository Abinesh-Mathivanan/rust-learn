fn main(){
    let mut v: Vec<i32> = (0..5).collect();
    let fat_p = &v[2..];
    let val = &v[1];
    println!("{:?}", v);
    println!("{}", val);
    println!("{:?}", fat_p);
    for num in v {
        println!("{}", num);
    }
}