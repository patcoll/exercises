pub fn diagonal_difference(arr: &[Vec<i32>]) -> Option<i32> {
    let num_rows = arr.len();
    let num_columns = arr[0].len();

    if num_rows == num_columns {
        let flattened: Vec<i32> = arr.iter().flatten().cloned().collect::<Vec<_>>();

        let first_start = 0;

        let first_step = num_rows + 1;

        let first_sum: i32 = flattened.iter().skip(first_start).step_by(first_step).take(num_rows).sum();

        let second_start = num_rows - 1;

        let second_step = num_rows - 1;

        let second_sum: i32 = flattened.iter().skip(second_start).step_by(second_step).take(num_rows).sum();

        let diff = first_sum - second_sum;

        Some(diff.abs())
    }
    else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        assert_eq!(diagonal_difference(&[vec![1,2,3], vec![4,5,6], vec![9,8,9]]), Some(2));
        assert_eq!(diagonal_difference(&[vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]]), Some(15));
    }
}
