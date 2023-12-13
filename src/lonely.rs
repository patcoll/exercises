pub fn find_lonely_integer(arr: &[i32]) -> Option<i32> {
    // will only work on odd-length vectors
    if arr.len() % 2 == 1 {
        let mut sorted: Vec<i32> = arr.to_owned();
        sorted.sort();

        (0..arr.len())
        .step_by(2)
        .find_map(|index| peek_lonely_integer(&sorted, index))
        .map(|index| sorted[index])
    }
    else {
        None
    }
}

fn peek_lonely_integer(arr: &[i32], peek_at: usize) -> Option<usize> {
    let peek_el = do_peek(arr, peek_at);
    let next_el = do_peek(arr, peek_at + 1);

    if peek_el != next_el {
        Some(peek_at)
    }
    else {
        None
    }
}

fn do_peek(arr: &[i32], peek_at: usize) -> Option<i32> {
    if peek_at < arr.len() {
        Some(arr[peek_at])
    }
    else {
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lonely_integer() {
        assert_eq!(find_lonely_integer(&[1,2,3,4,3,2,1]), Some(4));
        assert_eq!(find_lonely_integer(&[1,2,3,4,4,3,1]), Some(2));
        assert_eq!(find_lonely_integer(&[1,3]), None);
    }
}
