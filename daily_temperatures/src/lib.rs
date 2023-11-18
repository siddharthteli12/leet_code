pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len());
    let mut result = vec![0; temperatures.len()];
    for (index, temperature) in temperatures.iter().enumerate() {
        match stack.is_empty() {
            true => {
                stack.push((index, *temperature));
            }
            false => {
                let mut counter = 0;
                while counter < stack.len() {
                    if stack[counter].1 < *temperature {
                        let value: (usize, i32) = stack.remove(counter);
                        result[value.0] = (index - value.0) as i32;
                    } else {
                        counter += 1;
                    }
                }
                stack.push((index, *temperature));
            }
        }
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
    fn little_complicated() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            daily_temperatures(temperatures),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
