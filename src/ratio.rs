pub fn plus_minus(arr: &[i32]) -> ((i32, i32), (i32, i32), (i32, i32)) {
    let count: i32 = arr.len() as i32;

    let (pos, neg, zero) =
        arr.iter().fold((0, 0, 0), |(pos, neg, zero), &x| {
            (
                pos + (if x > 0 { 1 } else { 0 }),
                neg + (if x < 0 { 1 } else { 0 }),
                zero + (if x == 0 { 1 } else { 0 })
            )
        });

    (
        (pos, count),
        (neg, count),
        (zero, count)
    )
}

pub fn plus_minus_string(arr: &[i32]) -> String {
    let (pos, neg, zero) = plus_minus(arr);
    let decimal_places = 6;

    format!(
        "{:.places$}\n{:.places$}\n{:.places$}",
        (pos.0 as f64 / pos.1 as f64),
        (neg.0 as f64 / neg.1 as f64),
        (zero.0 as f64 / zero.1 as f64),
        places=decimal_places
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus_minus() {
        assert_eq!(plus_minus(&[1, 1, 0, -1, -1]), ((2, 5), (2, 5), (1, 5)));
        assert_eq!(plus_minus(&[-4, 3, -9, 0, 4, 1]), ((3, 6), (2, 6), (1, 6)));
        assert_eq!(plus_minus(&[1, 2, 3, -1, -2, -3, 0, 0]), ((3, 8), (3, 8), (2, 8)));
    }

    #[test]
    fn test_plus_minus_string() {
        assert_eq!(plus_minus_string(&[1, 1, 0, -1, -1]), "0.400000\n0.400000\n0.200000");
        assert_eq!(plus_minus_string(&[-4, 3, -9, 0, 4, 1]), "0.500000\n0.333333\n0.166667");
        assert_eq!(plus_minus_string(&[1, 2, 3, -1, -2, -3, 0, 0]), "0.375000\n0.375000\n0.250000");
    }
}
