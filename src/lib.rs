pub fn rgb(r: i32, g: i32, b: i32) -> String {
    // check each to see if single char, at which point add to format
    format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}

fn to_hex(num: i32) -> String {
    if num < 0 { return String::from("00") }
    if num > 255 { return String::from("FF") }
    let conv: String = format!("{:X}", num);
    if conv.len() == 1 {
        return "0".to_owned() + &conv
    }
    println!("input: {}\nconverted: {:X}", num, num);
    return conv
}