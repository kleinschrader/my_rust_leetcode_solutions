fn main() {}

#[allow(dead_code)]
pub fn largest_odd_number(num: String) -> String {
    let length_of_string = num.len();
    let mut first_odd = Option::None;

    for c in (0..length_of_string).rev() {
        let is_odd = (num.as_bytes()[c] & 1) == 1;

        if is_odd {
            first_odd = Some(c);
            break;
        }
    }

    match first_odd {
        Some(r) => num[0..(r + 1)].to_string(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::largest_odd_number;

    #[test]
    fn test_case_1() {
        let r = largest_odd_number("52".to_string());

        assert_eq!(r, "5");
    }

    #[test]
    fn test_case_2() {
        let r = largest_odd_number("4206".to_string());

        assert_eq!(r, "");
    }

    #[test]
    fn test_case_3() {
        let r = largest_odd_number("35427".to_string());

        assert_eq!(r, "35427");
    }
}
