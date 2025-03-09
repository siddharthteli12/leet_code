pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    // Will start at 0.
    let mut max = 0;
    let mut sum = 0;
    for val in gain {
        sum += val;
        max = max.max(sum);
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work() {
        let list = vec![-5, 1, 5, 0, -7];
        assert_eq!(largest_altitude(list), 1);
    }

    #[test]
    fn should_work2() {
        let list = vec![-4, -3, -2, -1, 4, 3, 2];
        assert_eq!(largest_altitude(list), 0);
    }
}
