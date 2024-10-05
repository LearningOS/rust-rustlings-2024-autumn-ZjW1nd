// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.


type Result<T> = std::result::Result<T, DivisionError>;//自定义的Result,md一直不过，可能是标准库只支持std err

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}
// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    else if a % b != 0 {
        return Err(DivisionError::NotDivisible(NotDivisibleError { dividend: a, divisor: b }));
    }
    Ok(a / b)
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results : Vec<Result<i32>>= numbers.into_iter().map(|n| divide(n, 27)).collect();
    //如果vec中所有元素都成功了就返回一个Ok打包的vec,否则返回DivisionError‘
    let mut result = Vec::new();
    for i in division_results {
        match i {
            Ok(n) => result.push(n),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results : Vec<Result<i32>> = numbers.into_iter().map(|n| divide(n, 27)).collect();//这个应该collect起来就行
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
