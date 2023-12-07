pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut right = *piles.iter().max().unwrap();
    let mut left = 1;
    let mut init_result = 0;
    let mut mid: i32 = (left + right) / 2;
    let mut h_required: i32 = cal(&piles, mid);
    while right >= left {
        println!("Left - {left}, Mid - {mid}, Right - {right} & h_required - {h_required}, h - {h}");
        if h_required > h {
            left = mid + 1;
        } else {
            init_result = mid;
            right = mid - 1;
        }
        mid = (left + right) / 2;
        h_required = cal(&piles, mid);
    }
    init_result
}

fn cal(list: &Vec<i32>, mid: i32) -> i32 {
        list
            .iter()
            .map(|&val| (val as f64 / mid as f64).ceil() as i32)
            .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        assert_eq!(min_eating_speed(piles, h), 4);
    }

    #[test]
    fn it_works_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        assert_eq!(min_eating_speed(piles, h), 30);
    }

    #[test]
    fn it_works_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        assert_eq!(min_eating_speed(piles, h), 23);
    }

    #[test]
    fn it_works_4() {
        let piles = vec![312884470];
        let h = 312884470;
        assert_eq!(min_eating_speed(piles, h), 1);
    }

    #[test]
    fn it_works_5() {
        let piles = vec![19];
        let h = 18;
        assert_eq!(min_eating_speed(piles, h), 2);
    }
}
