fn main() {}

const FULL_WEEKS_WORTH: i32 = 1 + 2 + 3 + 4 + 5 + 6 + 7;
const DAY_VALUES: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
const LENGHT_OF_WEEK: i32 = 7;

#[allow(dead_code)]
pub fn total_money(n: i32) -> i32 {
    let full_weeks = n / LENGHT_OF_WEEK;
    let remaining_days = n % LENGHT_OF_WEEK;

    let mut money = 0;

    for w in 0..full_weeks {
        money += FULL_WEEKS_WORTH + (LENGHT_OF_WEEK * w)
    }

    for d in 0..remaining_days {
        money += full_weeks;
        money += DAY_VALUES[d as usize];
    }

    money
}

#[cfg(test)]
mod tests {
    use crate::total_money;

    #[test]
    fn test_case_1() {
        let r = total_money(4);

        assert_eq!(r, 10);
    }

    #[test]
    fn test_case_2() {
        let r = total_money(10);

        assert_eq!(r, 37);
    }

    #[test]
    fn test_case_3() {
        let r = total_money(20);

        assert_eq!(r, 96);
    }
}
