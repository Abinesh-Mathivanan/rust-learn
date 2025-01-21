use std::io;

// fn main(){
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("expected string"); 
//     let input_vec: Vec<&str> = input.trim().split_whitespace().collect(); 
//     print!("{:?}", input_vec[0]);
// }

/* struct Rectangle {
    height: i32,
    width: i32,
} 

impl Rectangle {
    fn new(height: i32, width: i32) -> Self {
        return Rectangle {height, width} 
    }

    fn area(&self) -> i32 {
        return self.height * self.width;
    }

    fn isflexible(&self, total: i32) -> bool{
        return self.area() >= total;
    }
}

fn main(){
    let rect = Rectangle::new(10, 30);
    println!("{}", rect.area());
    println!("{}", rect.isflexible(100));
} */ 

// fn binarysearch(arr: &Vec<i32>, size: usize, target: i32) -> bool {
//     let mut start = 0; 
//     let mut end = size - 1; 

//     while start <= end {
//         let mid = (start + end) / 2; 
//         if arr[mid] == target {
//             return true; 
//         }
//         else if arr[mid] < target {
//             start = mid + 1;
//         }
//         else {
//             end = mid - 1;
//         }
//     }
//     return false;
// }

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn sumofvector(v: &Vec<i32>) -> i32 {
    return v.iter().sum();
}

fn removeduplicates(v: Vec<i32>) -> Vec<i32> {
    let mut final_vec = Vec::new();
    for i in &v {
        if !final_vec.contains(i) {
            final_vec.push(*i);
        }
    }
    return final_vec;
}
fn main(){
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("expect");
    // let input_vec: Vec<i32> = input.trim().split_whitespace().flat_map(str::parse).collect();
    // let result: bool = binarysearch(&input_vec, input_vec.len(), 10);
    // print!("{}", result);

    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("expect");
    // let number: i32 = input.trim().parse().expect("expected");
    // let nthfibonacci = fibonacci(number);
    // print!("{}", nthfibonacci);

    let v = vec![1, 2, 3, 4, 4, 5, 6, 6];
    let ans = sumofvector(&v);
    let rem_vec = removeduplicates(v);
    println!("{}", ans);
    println!("{:?}", rem_vec);
    let string = "malayalam";
    let rev_string: String = string.chars().rev().collect();
    println!("{}", string == rev_string);

}