fn main(){
    let sliced_string = "beenscodes";
    let length = sliced_string.len();
    let first_half = &sliced_string[..length/2 - 1];
    let second_half = &sliced_string[length/2 - 1..];
    println!("{}", first_half);
    println!("{}", second_half);
    println!("{}", sliced_string);

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

    let samples = vec!["hi", "hi", "not hi"];
    let joined_samples = samples.join(",");
    let mut concat_samples = samples.concat();
    concat_samples.push_str("additional");
    println!("{}", joined_samples);
    println!("{}", concat_samples);

    let istrue = concat_samples.contains("addn");
    let isstarts = concat_samples.starts_with("hihi");
    println!("{}", istrue);
    println!("{}", isstarts);

    /*
     other functions such as trim(), replace(), split(), chars().count()
     */
}