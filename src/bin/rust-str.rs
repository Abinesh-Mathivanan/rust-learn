fn main(){
    let sliced_string = "beenscodes";
    let length = sliced_string.len();
    let first_half = &sliced_string[..length/2 - 1];
    let second_half = &sliced_string[length/2 - 1..];
    println!("{}", first_half);
    println!("{}", second_half);

    let raw_text = r#"this is cool // /n /t /r //&*///#$#/*#"" "as said by abinesh""#;
    let raw_multiline = r#" 
        goof night,
        goofy ah memes,
        as said by the me.
    "#;
    let concat_text = "beens".to_string() + "codes" + "not";
    println!("{}", raw_text);
    println!("{}", raw_multiline);
    println!("{}", concat_text);
}