pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len());
    let mut result = vec![0; temperatures.len()];
    for (index, temperature) in temperatures.iter().enumerate() {
        while let Some(&last) = stack.last() {
            if *temperature > last.1 {
                let value_index = index - last.0;
                result[last.0] = value_index as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push((index, *temperature));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_temperatures() {
        let temperatures = vec![73, 74];
        assert_eq!(daily_temperatures(temperatures), vec![1, 0]);
    }

    #[test]
    fn little_temperatures() {
        let temperatures = vec![30, 40, 50, 60];
        assert_eq!(daily_temperatures(temperatures), vec![1, 1, 1, 0]);
    }

    #[test]
    fn complicated() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
