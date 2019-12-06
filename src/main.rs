use std::convert::TryInto;
use std::io;
fn main() {
    println!("Enter a possible Armstrong number.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess = guess.trim();
    if !guess.chars().all(|x| char::is_ascii_digit(&x)) {
        println!("This is not a number");
    } else {
        let guess_digits: Vec<u32> = guess.chars().map(|x| x.to_digit(10).unwrap()).collect();
        println!("{}", arm_strong(guess_digits));
    }
}
fn arm_strong(digits: Vec<u32>) -> String {
    let mut length: u32 = digits.len().try_into().unwrap();
    let number = digits.iter().fold(0, |result, x| {
        length -= 1;
        result + x * 10u32.pow(length)
    });
    length = digits.len().try_into().unwrap();
    let is_arm_strong: bool = digits.iter().fold(0, |result, x| result + x.pow(length)) == number;
    let mut result: String = number.to_string();
    result += " is ";
    if !is_arm_strong {
        result += "not ";
    }
    result += "an Armstrong number, because ";
    result += &number.to_string();
    result += " ";
    if !is_arm_strong {
        result += "!";
    }
    result += "= ";
    for (i, val) in digits.iter().enumerate() {
        if i != 0 {
            result += "+ ";
        }
        result += &format!("{}^{} ", val, length);
    }
    if length != 1 {
        result += "= ";
        for (i, val) in digits.iter().enumerate() {
            if i != 0 {
                result += "+ ";
            }
            result += &format!("{} ", val.pow(length));
        }
    }
    result += "= ";
    result += &digits
        .iter()
        .fold(0, |result, x| result + x.pow(length))
        .to_string();
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arm_stronger() {
        assert_eq!(
            "9 is an Armstrong number, because 9 = 9^1 = 9",
            arm_strong([9].to_vec())
        );
        assert_eq!(
            "10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1 + 0 = 1",
            arm_strong([1, 0].to_vec())
        );
        assert_eq!(
            "153 is an Armstrong number, because 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153",
            arm_strong([1, 5, 3].to_vec())
        );
        assert_eq!(
            "154 is not an Armstrong number, because 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190",
            arm_strong([1, 5, 4].to_vec())
        );
    }
}
