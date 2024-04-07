use ::string_utils::string_utils::StringBuilder;

fn main() {
    let mut sb = StringBuilder::new();
    sb.append(" ");
    sb.append("hello");
    sb.append(" ");
    sb.append("world!");
    sb.trim();
    println!("{}", sb.to_string());
    println!("left 5 : {}", sb.left(5).expect("error occured"));
    println!("right 5 : {}", sb.right(6).expect("error occured"));
    match sb.to_string().ends_with("world!") {
        true => println!("it is all true"),
        false => println!("it is all false"),
    }

    match sb.to_string().ends_with("wor!") {
        true => println!("it is all true"),
        false => println!("it is all false"),
    }
}
