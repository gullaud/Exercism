#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

use std::result::Result;

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base <= 1 {
        return  Result::Err(Error::InvalidOutputBase)
    }
    if from_base <= 1 {
        return  Result::Err(Error::InvalidInputBase)
    }
    if number.is_empty() {
        return Result::Ok(vec![0]);
    }
    let mut sum:u32 = 0;
    for i in 0..number.len() {
        let num = number[number.len()-1-i];
        if num >= from_base {
            return Result::Err(Error::InvalidDigit(num));
        }
        sum += num * from_base.pow(i.try_into().expect("too large"));
    }
    if sum == 0 {
        return Result::Ok(vec![0]);
    }
    let mut quotient = sum;
    let mut res = <Vec<u32>>::new();
    while quotient > 0 {
        res.push(quotient % to_base);
        quotient = quotient / to_base;
    }
    Result::Ok(res.into_iter().rev().collect::<Vec<u32>>())
}
