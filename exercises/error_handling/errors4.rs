// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // 检查是否为负数
        if value < 0 {
            return Err(CreationError::Negative);
        }
        // 检查是否为零
        if value == 0 {
            return Err(CreationError::Zero);
        }
        // 如果既不是负数也不是零，则转换为 `u64` 并返回 `Ok`
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}



// #[derive(PartialEq, Debug)]
// struct PositiveNonzeroInteger(u64);

// #[derive(PartialEq, Debug)]
// enum CreationError {
//     Negative,
//     Zero,
// }

// impl PositiveNonzeroInteger {
//     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
//         // Hmm...? Why is this only returning an Ok value?
//         Ok(PositiveNonzeroInteger(value as u64))
//     }
// }

// #[test]
// fn test_creation() {
//     assert!(PositiveNonzeroInteger::new(10).is_ok());
//     assert_eq!(
//         Err(CreationError::Negative),
//         PositiveNonzeroInteger::new(-10)
//     );
//     assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
// }
