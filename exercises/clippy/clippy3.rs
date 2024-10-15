// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.




fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some() {
        // 这里不会执行，因为 my_option 是 None
        panic!("This should not happen.");
    } else {
        // 如果 my_option 是 None，则打印信息
        println!("my_option is None.");
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 Rust 的模式匹配来交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}



// #[allow(unused_variables, unused_assignments)]
// fn main() {
//     let my_option: Option<()> = None;
//     if my_option.is_none() {
//         my_option.unwrap();
//     }

//     let my_arr = &[
//         -1, -2, -3
//         -4, -5, -6
//     ];
//     println!("My array! Here it is: {:?}", my_arr);

//     let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
//     println!("This Vec is empty, see? {:?}", my_empty_vec);

//     let mut value_a = 45;
//     let mut value_b = 66;
//     // Let's swap these two!
//     value_a = value_b;
//     value_b = value_a;
//     println!("value a: {}; value b: {}", value_a, value_b);
// }
