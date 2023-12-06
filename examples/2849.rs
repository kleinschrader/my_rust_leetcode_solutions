fn main() {

}

#[allow(dead_code)]
fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let rx = i32::abs(sx - fx);
    let ry = i32::abs(sy - fy);

    if (rx + ry) == 0 && (t == 1) {
        return false;
    }

    let diag_steps = i32::min(rx, ry);
    let longest_relative = i32::max(rx, ry);

    let straight_steps = longest_relative - diag_steps;

    let total_steps = diag_steps + straight_steps;

   !(t < total_steps)
}

#[cfg(test)]
mod tests {
    use crate::is_reachable_at_time;

    #[test]
    fn test_case_1() {
        let r = is_reachable_at_time(2, 4, 7,7, 6);
        assert_eq!(r, true);
    }

    #[test]
    fn test_case_2() {
        let r = is_reachable_at_time(3, 1, 7, 3, 3);
        assert_eq!(r, false);
    }

    #[test]
    fn test_case_3() {
        let r =is_reachable_at_time(1, 1, 1, 2, 0);
        assert_eq!(r, false);
    }

    #[test]
    fn test_case_4() {
        let r =is_reachable_at_time(1, 2, 1, 2, 1);
        assert_eq!(r, false);
    }

    #[test]
    fn test_case_5() {
        let r =is_reachable_at_time(1, 1, 1, 3, 2);
        assert_eq!(r, true);
    }
}