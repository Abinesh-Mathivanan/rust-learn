use std::rc::Rc;

fn main(){

    /* 
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error");
    let trim_input = input.trim();
    println!("{}", input.capacity());
    println!("{}", trim_input.len()); */

    /* struct Person {
        name: String,
        id: i32
    }

    let mut population = Vec::new();
    population.push(Person {name: "beens".to_string(), id: 1});
    population.push(Person {name: "her".to_string(), id: 2});
    for val in &population {
        println!("{} {}", val.name, val.id);
    } */ 

    /* let t = 5;
    let s = t; 
    let input = [0, 1, 2, 3, 4];
    let dup = input;
    println!("{}", t);
    println!("{}", s);
    println!("{:?}", input);
    println!("{:?}", dup); */ 

    /* let s: Rc<String> = Rc::new("beens".to_string());
    let t = s.clone();
    let u = s.clone(); 

    println!("{}", s);
    println!("{}", t);
    println!("{}", u);

    println!("{}", Rc::strong_count(&s)); // to print the no of reference counts */ 

    /* let s = 5;
    let t = 6;

    let rs = &s;
    let rt = &t; 

    let rrs = &rs; 
    let rrt = &rt; 

    if **rrt == t {
        println!("true");
    }
    else {
        println!("false");
    } */

    /* fn factorial(n: usize) -> usize {
        return (1..n+1).product();
    }

    let r = factorial(6);
    println!("{}", r + 5);

    println!("{:p}", &r); // used to print pointer variable */ 

    let v = vec![1, 2, 3, 4];
    let r = &v[1];
    println!("{:p} {:p}", &v, r);

    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r }
        }
        s
    }

    let v = [1, 2, 0, 4];
    let ans = smallest(&v);
    print!("{}", ans);

}