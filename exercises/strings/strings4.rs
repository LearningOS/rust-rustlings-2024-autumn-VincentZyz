// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // 传入 &str 类型
    string_slice("blue");

    // 传入 String 类型
    string(String::from("red"));

    // 传入 String 类型
    string(String::from("hi"));

    // 传入 String 类型
    string(String::from("rust is fun!"));

    // 传入 String 类型
    string(String::from("nice weather"));

    // 传入 String 类型
    string(format!("Interpolation {}", "Station"));

    // 传入 &str 类型
    string_slice(&String::from("abc")[0..1]);

    // 传入 &str 类型
    string_slice("  hello there ".trim());

    // 传入 String 类型
    string(String::from("Happy Monday!").replace("Mon", "Tues"));

    // 传入 String 类型
    string(String::from("mY sHiFt KeY iS sTiCkY").to_lowercase());
}
